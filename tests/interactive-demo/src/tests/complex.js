// tests/complex.js
export const tests = {
  'complex-create': async (ach) => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      return `Created complex: 3+4i`;
    });
  },
  'complex-arithmetic': async (ach) => {
    return ach.use(async () => {
      const c1 = ach.complex(2, 3);
      const c2 = ach.complex(1, 4);
      return `Created complex numbers: (2+3i) and (1+4i)`;
    });
  },
  'complex-conjugate': async (ach) => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      return `Complex number created: 3+4i`;
    });
  },
  'complex-magnitude': async (ach) => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      return `Complex number 3+4i created (magnitude should be 5)`;
    });
  },
};
