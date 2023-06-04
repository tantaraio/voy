const append = (box) => (str) => {
  const input = Array.isArray(str) ? str : [str];

  input.forEach((s) => {
    const para = document.createElement("p");
    const text = document.createTextNode(s);

    para.appendChild(text);
    box.appendChild(para);
  });
};
const intro = document.querySelector("#intro");
const index = document.querySelector("#index");
const resource = document.querySelector("#resource");

export const logIntro = append(intro);
export const logIndex = append(index);
export const logResource = append(resource);
