// tests/basic.js
export const tests = {
  'basic-arithmetic': async (ach) => {
    return ach.use(async () => {
      const sum = 10 + 5;
      const diff = 10 - 5;
      const prod = 10 * 5;
      const quot = 10 / 5;
      return `10+5=${sum}, 10-5=${diff}, 10*5=${prod}, 10/5=${quot}`;
    });
  },
  'basic-power': async (ach) => {
    return ach.use(async () => {
      const power = Math.pow(2, 8);
      const sqrt = Math.sqrt(16);
      return `2^8=${power}, √16=${sqrt}`;
    });
  },
  'basic-trig': async (ach) => {
    return ach.use(async () => {
      const angle = Math.PI / 4;
      const sinVal = ach.sin(angle);
      const cosVal = ach.cos(angle);
      const tanVal = ach.tan(angle);
      return `sin(π/4)=${sinVal.toFixed(4)}, cos(π/4)=${cosVal.toFixed(4)}, tan(π/4)=${tanVal.toFixed(4)}`;
    });
  },
  'basic-exp-log': async (ach) => {
    return ach.use(async () => {
      const exp = ach.exp(2);
      const ln = ach.ln(Math.E);
      return `e^2=${exp.toFixed(4)}, ln(e)=${ln.toFixed(4)}`;
    });
  },
  'basic-abs': async (ach) => {
    return ach.use(async () => {
      const abs = ach.abs(-42);
      return `|-42|=${abs}`;
    });
  },
};
