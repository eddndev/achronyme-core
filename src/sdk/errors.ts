/**
 * Base error class for all Achronyme SDK errors
 */
export class AchronymeError extends Error {
  constructor(message: string, public readonly code?: string) {
    super(message);
    this.name = 'AchronymeError';
    Object.setPrototypeOf(this, AchronymeError.prototype);
  }
}

/**
 * Thrown when there's a syntax error in the expression
 */
export class AchronymeSyntaxError extends AchronymeError {
  constructor(message: string, public readonly expression?: string) {
    super(message, 'SYNTAX_ERROR');
    this.name = 'AchronymeSyntaxError';
    Object.setPrototypeOf(this, AchronymeSyntaxError.prototype);
  }
}

/**
 * Thrown when a runtime error occurs during evaluation
 */
export class AchronymeRuntimeError extends AchronymeError {
  constructor(message: string, public readonly expression?: string) {
    super(message, 'RUNTIME_ERROR');
    this.name = 'AchronymeRuntimeError';
    Object.setPrototypeOf(this, AchronymeRuntimeError.prototype);
  }
}

/**
 * Thrown when a type mismatch occurs
 */
export class AchronymeTypeError extends AchronymeError {
  constructor(
    message: string,
    public readonly expected?: string,
    public readonly received?: string
  ) {
    super(message, 'TYPE_ERROR');
    this.name = 'AchronymeTypeError';
    Object.setPrototypeOf(this, AchronymeTypeError.prototype);
  }
}

/**
 * Thrown when an operation is attempted on a disposed value
 */
export class AchronymeDisposedError extends AchronymeError {
  constructor(message: string = 'Cannot operate on disposed value') {
    super(message, 'DISPOSED_ERROR');
    this.name = 'AchronymeDisposedError';
    Object.setPrototypeOf(this, AchronymeDisposedError.prototype);
  }
}

/**
 * Thrown when the WASM module is not initialized
 */
export class AchronymeNotInitializedError extends AchronymeError {
  constructor(message: string = 'Achronyme module not initialized. Call init() first.') {
    super(message, 'NOT_INITIALIZED');
    this.name = 'AchronymeNotInitializedError';
    Object.setPrototypeOf(this, AchronymeNotInitializedError.prototype);
  }
}

/**
 * Thrown when a function receives invalid arguments
 */
export class AchronymeArgumentError extends AchronymeError {
  constructor(
    message: string,
    public readonly functionName?: string,
    public readonly expectedArity?: number,
    public readonly receivedArity?: number
  ) {
    super(message, 'ARGUMENT_ERROR');
    this.name = 'AchronymeArgumentError';
    Object.setPrototypeOf(this, AchronymeArgumentError.prototype);
  }
}

/**
 * Parse C++ error message and wrap it in appropriate error class
 */
export function wrapCppError(cppError: string, expression?: string): AchronymeError {
  const errorLower = cppError.toLowerCase();

  // Syntax errors
  if (errorLower.includes('syntax') || errorLower.includes('parse')) {
    return new AchronymeSyntaxError(cppError, expression);
  }

  // Type errors
  if (errorLower.includes('type') || errorLower.includes('incompatible')) {
    return new AchronymeTypeError(cppError);
  }

  // Argument errors
  if (errorLower.includes('arity') || errorLower.includes('argument') || errorLower.includes('parameter')) {
    return new AchronymeArgumentError(cppError);
  }

  // Runtime errors
  if (errorLower.includes('runtime') || errorLower.includes('division by zero') || errorLower.includes('undefined')) {
    return new AchronymeRuntimeError(cppError, expression);
  }

  // Default to generic runtime error
  return new AchronymeRuntimeError(cppError, expression);
}
