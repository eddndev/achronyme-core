import fs from 'fs';
import path from 'path';

const functionsFile = 'C:\\apache\\htdocs\\achronyme-core\\crates\\achronyme-eval\\src\\functions.rs';
const content = fs.readFileSync(functionsFile, 'utf-8');

// Extract all function implementations
const functionPattern = /^fn (\w+)\(args: &\[Value\]\) -> Result<Value, String> \{[\s\S]*?^}/gm;
const matches = [...content.matchAll(functionPattern)];

console.log(`Found ${matches.length} functions`);

// Group functions by category based on registration order
const categories = {
  trig: ['sin', 'cos', 'tan', 'asin', 'acos', 'atan', 'atan2', 'sinh', 'cosh', 'tanh'],
  exponential: ['exp', 'ln', 'log10', 'log2', 'sqrt', 'cbrt', 'pow'],
  rounding: ['floor', 'ceil', 'round', 'trunc', 'abs', 'sign', 'deg', 'rad', 'min', 'max'],
  complex_ops: ['complex', 'real', 'imag', 'conj', 'arg'],
  vector_ops: ['dot', 'cross', 'norm', 'normalize'],
  stats: ['sum', 'mean', 'std'],
  dsp: ['fft', 'ifft', 'fft_mag', 'fft_phase', 'conv', 'conv_fft', 'hanning', 'hamming', 'blackman', 'rectangular', 'linspace'],
  matrix_ops: ['transpose', 'det', 'trace']
};

// List all extracted function names
console.log('\nExtracted functions:');
matches.forEach(match => {
  console.log(`- ${match[1]}`);
});

console.log('\n✓ Script completed');
