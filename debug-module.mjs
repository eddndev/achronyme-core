// Debug: Check what the module exports
const imported = await import('./dist/achronyme-core.js');

console.log('Imported object keys:', Object.keys(imported));
console.log('Has default?:', imported.default !== undefined);
console.log('Type of default:', typeof imported.default);
console.log('Type of imported:', typeof imported);

if (typeof imported.default === 'function') {
    console.log('\nTrying to call imported.default()...');
    try {
        const Module = await imported.default();
        console.log('Success! Module loaded');
        console.log('Module has eval?:', typeof Module.eval);

        if (typeof Module.eval === 'function') {
            const result = Module.eval('2 + 3');
            console.log('Test eval("2 + 3"):', result);
        }
    } catch (e) {
        console.error('Error:', e.message);
    }
}
