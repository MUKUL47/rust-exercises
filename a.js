function iterate(node, tree = treeNode, init = false) {
  if (init) {
    tree["type"] = Array.isArray(node) ? "array" : typeof node;
    tree["children"] = [];
  }
  let target = init ? tree.children : tree;
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
      iterate(node[key], target[target.length - 1].children);
    }
  }
}
class _Validator {
  data;
  object(obj = {}, flag) {
    let children = [];
    for (let key in obj) {
      children.push({ name: key, ...obj[key] });
    }
    this.data = { type: "object", children };
  }
  array(...children) {
    if (!Array.isArray(children)) throw "NASD";
    this.data = { type: "array", children };
  }
  static object(obj = {}, flag) {
    if (Array.isArray(obj) || typeof obj !== "object")
      throw new Error("Only object(s) allowed");
    let children = [];
    for (let key in obj) {
      children.push({ name: key, ...obj[key] });
    }
    return { type: "object", children: children.length ? children : [] };
  }
  static array(...children) {
    return {
      type: "array",
      children,
    };
  }
  static string() {
    return {
      type: "string",
    };
  }
  static number() {
    return {
      type: "number",
    };
  }
  static required(child) {
    return { ...child, required: true };
  }
}
// console.log(JSON.stringify(v.data, 3, null));
// console.log(JSON.stringify(treeNode, null, 1));

function joinBranches(
  node,
  prefix = "JSON",
  totalNodes = [],
  isSchema = false
) {
  let children = node.children;
  const isA = node.type === "array";
  children?.forEach((child, index) => {
    const DEFAULT = isA && isSchema ? index : child?.name || child?.key || "_";
    let v = `${prefix}.${DEFAULT}`;
    totalNodes.push([v, child]);
    joinBranches(child, v, totalNodes, isSchema);
  });
  totalNodes = [["JSON", { type: node.type }], ...totalNodes];
  return totalNodes;
}

function union(joints) {
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
function validate(schemaJson, payloadJson) {
  const objectSets = new Set();
  //find all where postfix is for object
  for (let key in schemaJson) {
    if (key.slice(key.length - 1) != "_") {
      objectSets.add(key);
    }
  }
  //iterate payload json and cross check with schema json
  for (let key in payloadJson) {
    const isObject = key.slice(key.length - 1) != "_";
    const targetKeySchema = schemaJson[key];
    if (!targetKeySchema) {
      return `${key} not found in schema`;
    }
    if (isObject && targetKeySchema[0].type != payloadJson[key][0].type) {
      return `expected type ${targetKeySchema[0].type} but found ${payloadJson[key][0].type} at ${key}`;
    }
    if (!isObject) {
      //is array cross check with
      const targetMatch = schemaJson[key];
      const source = payloadJson[key];
      let _break = false;
      for (let m of targetMatch) {
        for (let s of source) {
          if (m.type == s.type) {
            _break = true;
            break;
          }
        }
      }
      if (!_break) return `No valid array item found at ${key}`;
    } else {
      objectSets.delete(key);
    }
  }
  if (objectSets.size > 0) {
    return `Following properties missing from schema ${Array.from(
      objectSets
    ).join(", ")}`;
  }
  return true;
}
//////////////////////////////

const v = new Validator();
const fourLevelSchema = v.object({
  name: Validator.required(Validator.string()),
  values: Validator.array(Validator.string(), Validator.number()),
});
let arr3 = { name: "2", values: [{}] };
let treeNodeCorrect = {};
iterate(arr3, treeNodeCorrect, !!1);
let schema = union(joinBranches(v.data, "JSON", []));
// let p = union(joinBranches(treeNodeCorrect, "JSON", [], true));
console.log(schema);
console.log(validator(treeNodeCorrect));
// console.log(validate(s, p));

function validator(node, prefix = "JSON", prefix_index = "JSON") {
  let children = node.children;
  if (!schema[prefix]) throw [node, prefix];
  const recursiveChildren = [];
  for (let i = 0; i < children.length; i++) {
    const child = children[i];
    const DEFAULT = node.type != "array" ? child?.key ?? child.name : "_";
    const DEFAULT_INDEX = child?.key ?? child.name;
    let key = `${prefix}.${DEFAULT}`;
    let key_index = `${prefix_index}.${DEFAULT_INDEX}`;
    const dataType = schema[key]?.find(
      (schemaData) => schemaData.type == child.type
    );
    if (!dataType) {
      throw `Expected ${schema[key]
        ?.map((v) => v.type)
        .join(" or ")
        .trim()} but found ${child.type} at ${key_index}`;
    }
    if (["array", "object"].includes(dataType.type)) {
      recursiveChildren.push([child, key, key_index]);
    }
  }
  recursiveChildren.forEach(([c, childPrefix, key_index]) =>
    validator(c, childPrefix, key_index)
  );
}
