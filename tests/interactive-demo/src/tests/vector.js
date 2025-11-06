// tests/vector.js
export const tests = {
  'vector-create': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([1, 2, 3, 4, 5]);
      return `Created vector with 5 elements`;
    });
  },
  'vector-arithmetic': async (ach) => {
    return ach.use(async () => {
      const v1 = ach.vector([1, 2, 3]);
      const v2 = ach.vector([4, 5, 6]);
      return `Created two vectors: v1=[1,2,3], v2=[4,5,6]`;
    });
  },
  'vector-dot': async (ach) => {
    return ach.use(async () => {
      const v1 = ach.vector([1, 2, 3]);
      const v2 = ach.vector([4, 5, 6]);
      const dot = ach.dot(v1, v2);
      return `[1,2,3] · [4,5,6] = ${dot}`;
    });
  },
  'vector-stats': async (ach) => {
    return ach.use(async () => {
      const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
      const meanVal = ach.mean(data);
      const stdVal = ach.std(data);
      return `mean=${meanVal}, std=${stdVal.toFixed(4)}`;
    });
  },
  'vector-operations': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([3, 1, 4, 1, 5, 9, 2, 6]);
      const sum = ach.sum(v);
      const max = ach.max(v);
      const min = ach.min(v);
      return `sum=${sum}, max=${max}, min=${min}`;
    });
  },
};
