const createNumDiv = (id, num) => {
  const numDiv = document.createElement('div');
  numDiv.classList.add('num');
  numDiv.innerText = `${id} = ${num}`;

  return numDiv;
};

const createOneDiv = (id, one) => {
  const oneDiv = document.createElement('div');
  oneDiv.classList.add('one', 'mat');

  const tagDiv = document.createElement('div');
  tagDiv.innerText = `${id} = `;
  oneDiv.appendChild(tagDiv);

  const dataDiv = document.createElement('div');
  dataDiv.classList.add('data');
  oneDiv.appendChild(dataDiv);

  const rowDiv = document.createElement('div');
  rowDiv.classList.add('row');
  dataDiv.appendChild(rowDiv);

  one.data.forEach(element => {
    const cellDiv = document.createElement('div');
    cellDiv.classList.add('cell');
    rowDiv.appendChild(cellDiv);

    cellDiv.innerText = element;
  });

  return oneDiv;
};

const createTwoDiv = (id, two) => {
  const twoDiv = document.createElement('div');
  twoDiv.classList.add('one', 'mat');

  const tagDiv = document.createElement('div');
  tagDiv.innerText = `${id} = `;
  twoDiv.appendChild(tagDiv);

  const dataDiv = document.createElement('div');
  dataDiv.classList.add('data');
  twoDiv.appendChild(dataDiv);

  two.data.forEach(row => {
    const rowDiv = document.createElement('div');
    rowDiv.classList.add('row');
    dataDiv.appendChild(rowDiv);

    row.forEach(element => {
      const cellDiv = document.createElement('div');
      cellDiv.classList.add('cell');
      rowDiv.appendChild(cellDiv);

      cellDiv.innerText = element;
    });
  });

  return twoDiv;
};

export const createDisplayDiv = (id, object) => {
  if (typeof object === 'number') {
    return createNumDiv(id, object);
  } else if (typeof object === 'object') {
    if (object.shape().length == 1) {
      return createOneDiv(id, object);
    } else if (object.shape().length == 2) {
      return createTwoDiv(id, object);
    }
  }
};
