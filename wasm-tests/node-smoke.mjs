import * as wasm from '../pkg-node/text_processing_rs.js';

function assertEqual(actual, expected, message) {
  if (actual !== expected) {
    throw new Error(`${message}: expected "${expected}", got "${actual}"`);
  }
}

function assertTrue(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

assertEqual(wasm.normalize('two hundred'), '200', 'normalize should convert spoken numbers');
assertEqual(
  wasm.normalizeWithLang('two hundred', 'en'),
  '200',
  'normalizeWithLang should work'
);
assertEqual(
  wasm.normalizeSentence('I have twenty one apples'),
  'I have 21 apples',
  'normalizeSentence should convert spans'
);
assertEqual(
  wasm.normalizeSentenceLang("j'ai vingt et un ans", 'fr'),
  "j'ai 21 ans",
  'normalizeSentenceLang should convert spans per language'
);
assertEqual(wasm.tnNormalize('$5.50'), 'five dollars fifty cents', 'tnNormalize should work');
assertEqual(
  wasm.tnNormalizeSentence('I paid $5 for 23 items'),
  'I paid five dollars for twenty three items',
  'tnNormalizeSentence should convert spans'
);

// Vietnamese (vi) — both ITN and TN directions.
assertEqual(
  wasm.normalizeWithLang('hai mươi mốt', 'vi'),
  '21',
  'vi ITN: cardinal with mốt positional variant'
);
assertEqual(
  wasm.normalizeSentenceLang('tôi có hai mươi mốt quả táo', 'vi'),
  'tôi có 21 quả táo',
  'vi ITN: sentence mode with embedded cardinal'
);
assertEqual(
  wasm.normalizeWithLang('năm nghìn đồng', 'vi'),
  '5000 ₫',
  'vi ITN: money expression'
);
assertEqual(
  wasm.normalizeWithLang('mười bốn giờ ba mươi', 'vi'),
  '14:30',
  'vi ITN: time expression'
);
assertEqual(
  wasm.tnNormalizeLang('123', 'vi'),
  'một trăm hai mươi ba',
  'vi TN: cardinal written to spoken'
);
assertEqual(
  wasm.tnNormalizeSentenceLang('Tôi có 123 quả táo', 'vi'),
  'Tôi có một trăm hai mươi ba quả táo',
  'vi TN: sentence mode with embedded cardinal'
);

wasm.clearRules();
assertEqual(wasm.ruleCount(), 0, 'ruleCount starts at 0');
wasm.addRule('gee pee tee', 'GPT');
assertEqual(wasm.ruleCount(), 1, 'ruleCount increments');
assertEqual(wasm.normalize('gee pee tee'), 'GPT', 'custom rules should apply');
assertTrue(wasm.removeRule('gee pee tee'), 'removeRule should return true when found');
assertEqual(wasm.ruleCount(), 0, 'rule removed');
assertTrue(!wasm.removeRule('gee pee tee'), 'removeRule should return false when missing');

console.log('WASM node smoke test passed');
