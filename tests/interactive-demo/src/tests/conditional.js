// tests/conditional.js - Conditional and Piecewise Function Tests
export const tests = {
  'conditional-abs-scalar': async (ach) => {
    return ach.use(async () => {
      const result1 = ach.conditional.absValue(-5);
      const result2 = ach.conditional.absValue(3);
      return `abs(-5)=${result1}, abs(3)=${result2}`;
    });
  },

  'conditional-abs-vector': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-5, -2, 0, 2, 5]);
      const result = ach.conditional.absVector(v);
      return `abs([-5,-2,0,2,5]) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-relu-scalar': async (ach) => {
    return ach.use(async () => {
      const pos = ach.conditional.reluValue(5);
      const neg = ach.conditional.reluValue(-3);
      const zero = ach.conditional.reluValue(0);
      return `relu(5)=${pos}, relu(-3)=${neg}, relu(0)=${zero}`;
    });
  },

  'conditional-relu-vector': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-5, -2, 0, 2, 5]);
      const result = ach.conditional.reluVector(v);
      return `relu([-5,-2,0,2,5]) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-leaky-relu': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-10, -5, 0, 5, 10]);
      const result = ach.conditional.leakyReluVector(v, 0.01);
      return `leaky_relu (α=0.01) = [${result.data.map(x => x.toFixed(2)).join(', ')}]`;
    });
  },

  'conditional-sign': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-10, -5, 0, 5, 10]);
      const result = ach.conditional.signVector(v);
      return `sign([-10,-5,0,5,10]) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-heaviside': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-2, -1, 0, 1, 2]);
      const result = ach.conditional.heavisideVector(v);
      return `heaviside([-2,-1,0,1,2]) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-clamp': async (ach) => {
    return ach.use(async () => {
      const v = ach.vector([-5, -1, 0, 1, 5]);
      const result = ach.conditional.clampVector(v, -1, 1);
      return `clamp([-5,-1,0,1,5], -1, 1) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-rect-pulse': async (ach) => {
    return ach.use(async () => {
      const t = ach.vector([-1, -0.5, 0, 0.5, 1]);
      const result = ach.conditional.rectVector(t, 1);
      return `rect(width=1) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-square-wave': async (ach) => {
    return ach.use(async () => {
      const t = ach.linspace(0, 2 * Math.PI, 8);
      const result = ach.conditional.squareWaveVector(t);
      return `square_wave (8 samples) = [${result.data.join(', ')}]`;
    });
  },

  'conditional-triangle-wave': async (ach) => {
    return ach.use(async () => {
      const t = ach.linspace(0, 4, 9);
      const result = ach.conditional.triangleWaveVector(t);
      return `triangle_wave (9 samples) = [${result.data.map(x => x.toFixed(2)).join(', ')}]`;
    });
  },

  'conditional-dsp-integration': async (ach) => {
    return ach.use(async () => {
      // Generate square wave and apply FFT
      const t = ach.linspace(0, 2 * Math.PI, 32);
      const square = ach.conditional.squareWaveVector(t);
      const spectrum = ach.fftMag(square);
      const dc = spectrum.data[0].toFixed(2);
      const fundamental = spectrum.data[1].toFixed(2);
      return `Square wave FFT: DC=${dc}, Fundamental=${fundamental}`;
    });
  },

  'conditional-chain-operations': async (ach) => {
    return ach.use(async () => {
      // Chain: generate data -> apply relu -> compute stats
      const v = ach.vector([-5, -2, 0, 2, 5]);
      const relu = ach.conditional.reluVector(v);
      const mean = ach.mean(relu);
      const sum = ach.sum(relu);
      return `relu then stats: mean=${mean.toFixed(2)}, sum=${sum}`;
    });
  },

  'conditional-combined-functions': async (ach) => {
    return ach.use(async () => {
      // Apply multiple conditional functions to same data
      const v = ach.vector([-3, -1, 0, 1, 3]);
      const absVec = ach.conditional.absVector(v);
      const signVec = ach.conditional.signVector(v);
      const heavVec = ach.conditional.heavisideVector(v);
      return `abs, sign, heaviside applied successfully`;
    });
  },

  'conditional-waveform-analysis': async (ach) => {
    return ach.use(async () => {
      // Generate triangle wave and analyze
      const t = ach.linspace(0, 4, 100);
      const tri = ach.conditional.triangleWaveVector(t);
      const min = ach.min(tri);
      const max = ach.max(tri);
      const mean = ach.mean(tri);
      return `Triangle: min=${min.toFixed(2)}, max=${max.toFixed(2)}, mean=${mean.toFixed(2)}`;
    });
  },
};
