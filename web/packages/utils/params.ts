/**
 * serialize any object
 * @param a - any object to serialize
 * @returns a serialized string
 */
export default function params(a: any): string {
  const s: string[] = [];
  const add = (k: string, v: any) => {
    v = typeof v === "function" ? v() : v;
    v = v === null ? "" : v === undefined ? "" : v;
    s[s.length] = encodeURIComponent(k) + "=" + encodeURIComponent(v);
  };
  const buildParams = (prefix: string, obj: any) => {
    let i, len, key;

    if (prefix) {
      if (Array.isArray(obj)) {
        for (i = 0, len = obj.length; i < len; i++) {
          buildParams(
            prefix +
              "[" +
              (typeof obj[i] === "object" && obj[i] ? i : "") +
              "]",
            obj[i],
          );
        }
      } else if (Object.prototype.toString.call(obj) === "[object Object]") {
        for (key in obj) {
          buildParams(prefix + "[" + key + "]", obj[key]);
        }
      } else {
        add(prefix, obj);
      }
    } else if (Array.isArray(obj)) {
      for (i = 0, len = obj.length; i < len; i++) {
        add(obj[i].name, obj[i].value);
      }
    } else {
      for (key in obj) {
        buildParams(key, obj[key]);
      }
    }
    return s;
  };

  return buildParams("", a).join("&");
}
