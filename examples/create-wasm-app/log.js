const append = (box) => (str) => {
  const input = Array.isArray(str) ? str : [str];

  input.forEach((s) => {
    const para = document.createElement("p");
    const text = document.createTextNode(s);

    para.appendChild(text);
    box.appendChild(para);
  });
};
const container = document.querySelector("#example");
export const log = append(container);
