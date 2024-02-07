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
    return new DataType({ type: "object", children });
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
        { ...child.value, children: child.value?.children?.length },
      ]);
      V.#joinBranches(child, v, totalNodes, isSchema);
    });
    totalNodes = [
      ["JSON", { ...node.value, children: node.value?.children?.length }],
      ...totalNodes,
    ];
    return totalNodes;
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
  constructor(schema) {
    this.schema = schema;
    for (let key in this.schema) {
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
      console.log(parentNode.name, node.length);
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
    this.requiredFields.forEach((value, key) => {
      if (value === 0) {
        throw `${key} not found`;
      }
    });
  }
  startValidation(node) {
    this.validate(node);
    this.checkRequiredFields();
  }
  validate(node, prefix = "JSON", prefix_index = "JSON") {
    let children = node.children;
    if (prefix === "JSON" && this.schema[prefix][0].type != node.type)
      throw `Expected ${this.schema[prefix]
        ?.map((v) => v.type)
        .join(" or ")
        .trim()} but found ${node.type} at ${prefix}`;
    const recursiveChildren = [];
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
      if (!dataType) {
        throw `Expected ${this.schema[key]
          ?.map((v) => v.type)
          .join(" or ")
          .trim()} but found ${child.type} at ${key_index}`;
      }
      if (this.requiredFields.has(`${key}-${dataType.type}`)) {
        this.requiredFields.set(
          `${key}-${dataType.type}`,
          this.requiredFields.get(`${key}-${dataType.type}`) + 1
        );
      }
      if (["array", "object"].includes(dataType.type)) {
        console.log(
          `${prefix_index}.${DEFAULT_INDEX}`,
          "------",
          `${prefix}.${DEFAULT}`,
          "-----------",
          dataType.type
        );
        if (child.type === "array") {
          this.arrayFields.set(
            key,
            !this.arrayFields.has(key)
              ? [{ length: child.length, key_index }]
              : [
                  ...this.arrayFields.get(key),
                  { length: child.length, key_index },
                ]
          );
        }
        recursiveChildren.push([child, key, key_index]);
      }
    }
    recursiveChildren.forEach(([c, childPrefix, key_index]) =>
      this.validate(c, childPrefix, key_index)
    );
  }
}
let v = V.object({
  data: V.array(
    V.number(),
    V.object({
      contacts: V.array(
        V.object({
          type: V.string(),
          value: V.object({
            k: V.object({ m: V.array(V.string()).required() }),
          }),
          // types: V.array(V.object({ k: V.string().required() })),
        }).required()
      ),
    })
      // .allowUnknown()
      .required()
  ),
});

const schema = V.generateSchema(v);
const vv = new Validator(schema);
let treeNodeCorrect = {};
let arr = {
  data: [
    {
      contacts: [
        {
          value: { k: { m: ["123"] } },
        },
        {},
        { value: {} },
      ],
    },
    {
      contacts: [
        {
          value: { k: { m: ["123"] } },
        },
      ],
    },
    { contacts: [] },
    {},
    2,
  ],
};
console.log(vv.validateJson(arr, treeNodeCorrect, !!1));
vv.startValidation(treeNodeCorrect);
console.log(vv.requiredFields);
console.log(vv.arrayFields);
console.log(vv.childParentSet);
