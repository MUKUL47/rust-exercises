class DataType {
  value;
  constructor(value) {
    this.value = value;
    return this;
  }
  required() {
    this.value.required = true;
    return this;
  }
  example(example) {
    this.value.example = example;
    return this;
  }
  allowUnknown() {
    if (this.value.type !== "object") {
      return this;
    }
    this.value.allowUnknown = true;
    return this;
  }
}
class V {
  static objectKeysMap = new Map();
  static any() {
    return new DataType({ type: "any" });
  }
  static string() {
    return new DataType({ type: "string" });
  }
  static number() {
    return new DataType({ type: "number" });
  }
  static boolean() {
    return new DataType({ type: "boolean" });
  }
  static array(...children) {
    return new DataType({ type: "array", children });
  }
  static object(obj) {
    if (!obj) {
      obj = {};
    }
    if (Array.isArray(obj) || typeof obj !== "object")
      throw new Error("[object] Only object(s) allowed");
    let children = [];
    for (let key in obj) {
      children.push({ name: key, ...obj[key] });
    }
    return new DataType({
      type: "object",
      children,
    });
  }

  static #joinBranches(
    node,
    prefix = "JSON",
    totalNodes = [],
    isSchema = false
  ) {
    let children = node.value?.children;
    const isA = node.type === "array";
    children?.forEach((child, index) => {
      const DEFAULT =
        isA && isSchema ? index : child?.name || child?.key || "_";
      let v = `${prefix}.${DEFAULT}`;
      totalNodes.push([
        v,
        {
          ...child.value,
          children: child.value?.children?.length,
        },
      ]);
      if (child.value.type === "object") {
        V.updateObjectKeys(v, child.value.children);
      }
      V.#joinBranches(child, v, totalNodes, isSchema);
    });
    totalNodes = [
      [
        "JSON",
        {
          ...node.value,
          children: node.value?.children?.length,
        },
      ],
      ...totalNodes,
    ];
    if (node.value.type === "object") {
      V.updateObjectKeys("JSON", node.value.children);
    }
    return totalNodes;
  }

  static updateObjectKeys(key, children) {
    V.objectKeysMap.set(
      key,
      children.map((v) => ({ ...v, required: !!v.value.required, value: null }))
    );
  }
  static #parseSchema(v) {
    if (!v.value?.children) throw new Error("Only Objects or Array!");
    return this.#joinBranches(v);
  }

  static #union(joints) {
    return joints.reduce((acc, cc) => {
      const key = cc[0];
      const value = cc[1];
      if (acc.hasOwnProperty(key)) {
        acc[key].push(value);
      } else {
        acc[key] = [value];
      }
      return acc;
    }, {});
  }

  static generateSchema(v) {
    return this.#union(this.#parseSchema(v));
  }
}

class Validator {
  requiredFields = new Map();
  unknownFields = new Set();
  childParentSet = new Map();
  arrayFields = new Map();
  objectStack = new Array();
  visitedRequiredFields = new Map();
  childParentRelationship = new Map();
  constructor(schema) {
    this.schema = schema;
    for (let key in this.schema) {
      this.childParentRelationship.set(key, this.schema?.[key]?.[0]?.parent);
      const filter = this.schema[key].filter((s) => s.required);
      filter.forEach((f) => {
        this.requiredFields.set(`${key}-${f.type}`, 0);
      });
      if (this.schema[key].length === 1 && this.schema[key][0].allowUnknown) {
        this.unknownFields.add(key);
      } //always valid for object & object will always have === 1 child
      else if (
        this.schema[key].length > 1 &&
        !!this.schema[key].filter((v) => !!v.allowUnknown).length
      ) {
        throw new Error(
          `[unknownFields] allowUnknown only works inside an object not array for: ${key}`
        );
      }
    }
  }
  validateJson(node, tree = {}, init = false, parentNode = tree) {
    if (init) {
      tree["type"] = Array.isArray(node) ? "array" : typeof node;
      tree["children"] = [];
    }
    let target = init ? tree.children : tree;
    if (parentNode?.type === "array") {
      parentNode.length = node.length;
    }
    for (let key in node) {
      const type = typeof node[key];
      if (type !== "object") {
        target.push({
          type,
          value: node[key],
          key,
        });
      } else if (type === "object") {
        target.push({
          type: Array.isArray(node[key]) ? "array" : "object",
          ...(Array.isArray(node) ? {} : { name: key }),
          children: [],
        });
        this.validateJson(
          node[key],
          target[target.length - 1].children,
          null,
          target[target.length - 1]
        );
      }
    }
    return tree;
  }

  checkRequiredFields() {
    // this.requiredFields.forEach((value, key) => {
    //   if (value === 0) {
    //     throw `${key} not found`;
    //   }
    // });
  }
  startValidation(node) {
    this.validate(node);
    this.checkRequiredFields();
  }
  validate(node, prefix = "JSON", prefix_index = "JSON", depth = 0) {
    let children = node.children;
    if (prefix === "JSON" && this.schema[prefix][0].type != node.type)
      throw `Expected ${this.schema[prefix]
        ?.map((v) => v.type)
        .join(" or ")
        .trim()} but found ${node.type} at ${prefix}`;
    const recursiveChildren = [];
    const objectKeys = [];
    for (let i = 0; i < children.length; i++) {
      const child = children[i];
      const DEFAULT = node.type != "array" ? child?.key ?? child.name : "_";
      const DEFAULT_INDEX =
        child?.key != undefined ? child?.key : child.name || i;
      let key = `${prefix}.${DEFAULT}`;
      this.childParentSet.set(key, prefix);

      let key_index = `${prefix_index}.${DEFAULT_INDEX}`;
      if (this.unknownFields.has(prefix) && !this.schema[key]) continue;
      if (!this.schema[key] && !this.unknownFields.has(prefix)) {
        throw `Unexpected key ${key_index} at ${prefix}`;
      }
      const dataType = this.schema[key]?.find(
        (schemaData) => schemaData.type == child.type
      );
      const currentSchemaTypes = this.schema[key]?.map((v) => v.type) || [];
      if (!dataType) {
        if (currentSchemaTypes[0] === "any") continue;
        throw `Expected ${currentSchemaTypes.join(" or ").trim()} but found ${
          child.type
        } at ${key_index}`;
      }
      const defaultKey = `${prefix}.${DEFAULT}`;
      const originalKey = `${prefix_index}.${DEFAULT_INDEX}`;
      if (["array", "object"].includes(dataType.type)) {
        recursiveChildren.push([child, key, key_index]);
      }
      if (node.type === "array") {
        objectKeys.push(child.type);
      } else {
        objectKeys.push(child.name ?? child.key);
      }
    }
    const isArray = node.type === "array";
    if (isArray) {
      const arrayKey = `${prefix}._`;
      const filteredRequiredKeys = this.schema[arrayKey].filter(
        (k) => k.required
      );
      const missingKeys = filteredRequiredKeys.filter(
        (obj) => !objectKeys.includes(obj.type)
      );
      if (missingKeys.length) {
        throw new Error(
          `Missing type(s) ${missingKeys
            .map((v) => v.type)
            .join(", ")} for ${prefix}`
        );
      }
    } else {
      const filteredRequiredKeys =
        V.objectKeysMap.get(prefix)?.filter((k) => k.required) || [];

      const missingKeys = filteredRequiredKeys.filter(
        (obj) => !objectKeys.includes(obj.name)
      );
      if (missingKeys.length) {
        throw new Error(
          `Missing keys(s) [${missingKeys
            .map((v) => v.name)
            .join(", ")}] for ${prefix_index}`
        );
      }
    }
    recursiveChildren.forEach(([c, childPrefix, key_index]) =>
      this.validate(c, childPrefix, key_index)
    );
  }
}
let v = V.object({
  category: V.string().required(),
  quantity: V.number().required(),
  items: V.array(
    V.object({
      itemName: V.string().required(),
      itemPrice: V.number().required(),
      subItems: V.array(
        V.object({
          subItemName: V.string().required(),
          subItemQuantity: V.number().required(),
          subItemDetails: V.array(V.string().required()).required(),
          customAttribute: V.any(),
        })
      ).required(),
      additionalInfo: V.any(),
    })
  ).required(),
  metadata: V.any(),
}).allowUnknown();
const schema = V.generateSchema(v);
const vv = new Validator(schema);
let treeNodeCorrect = {};
let arr = {
  category: "Electronics",
  quantity: 2,
  items: [
    {
      itemName: "Laptop",
      itemPrice: 1200,
      subItems: [
        {
          subItemName: "Processor",
          subItemQuantity: 1,
          subItemDetails: [],
          customAttribute: { cores: 8 },
        },
        {
          subItemName: "RAM",
          subItemQuantity: 2,
          subItemDetails: ["16GB DDR4"],
          customAttribute: { type: "Corsair" },
        },
      ],
      additionalInfo: { brand: "Dell" },
    },
    {
      itemName: "Smartphone",
      itemPrice: 800,
      subItems: [
        {
          subItemName: "Camera",
          subItemQuantity: 2,
          subItemDetails: ["12MP", "4K video"],
          customAttribute: { type: "Dual" },
        },
        {
          subItemName: "Battery",
          subItemQuantity: 1,
          subItemDetails: ["4000mAh"],
        },
      ],
      additionalInfo: { brand: "Samsung" },
    },
  ],
  metadata: { orderDate: "2024-02-14" },
  unexpectedKey: "This key is unexpected but allowed in the schema",
};

vv.validateJson(arr, treeNodeCorrect, !!1);
try {
  vv.startValidation(treeNodeCorrect);
} catch (e) {
  console.log(e);
}
console.log(vv.schema);
