import { Abi, InputMap } from '@noir-lang/noirc_abi';

export const abi: Abi = {
  parameters: [
    {
      name: 'foo',
      type: { kind: 'integer', sign: 'unsigned', width: 32 },
      visibility: 'private',
    },
  ],
  param_witnesses: { foo: [1] },
  return_type: null,
  return_witnesses: [],
};

export const inputs: InputMap = {
  foo: `0x${(1n << 38n).toString(16)}`,
};
