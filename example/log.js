const append = (box) => (str) => {
  const para = document.createElement("p");
  const text = document.createTextNode(str);

  para.appendChild(text);
  box.appendChild(para);
};
const container = document.querySelector("#example");
export const log = append(container);
