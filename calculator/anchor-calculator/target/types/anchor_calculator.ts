/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/anchor_calculator.json`.
 */
export type AnchorCalculator = {
  "address": "AYkipetpaY5p637YMQZZXB6668uvWF6RNQyvnixNb3W2",
  "metadata": {
    "name": "anchorCalculator",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "add",
      "discriminator": [
        41,
        249,
        249,
        146,
        197,
        111,
        56,
        181
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "signer",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "num",
          "type": "u32"
        }
      ]
    },
    {
      "name": "double",
      "discriminator": [
        162,
        214,
        74,
        72,
        133,
        109,
        203,
        102
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "signer",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "half",
      "discriminator": [
        76,
        180,
        225,
        131,
        93,
        32,
        187,
        25
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "signer",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "init",
      "discriminator": [
        220,
        59,
        207,
        236,
        108,
        250,
        47,
        100
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "initVal",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "dataShape",
      "discriminator": [
        53,
        175,
        45,
        58,
        248,
        207,
        235,
        106
      ]
    }
  ],
  "types": [
    {
      "name": "dataShape",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "num",
            "type": "u32"
          }
        ]
      }
    }
  ]
};
