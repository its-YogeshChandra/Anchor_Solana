/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/class_cal.json`.
 */
export type ClassCal = {
  "address": "J3gWhe7BHUuVTSg8HNcEEPzaZYTMLL4oHWJtCDjKp4LD",
  "metadata": {
    "name": "classCal",
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
          "name": "accounts",
          "writable": true
        },
        {
          "name": "signer",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "i32"
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
          "name": "accounts",
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
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
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
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "accountShape",
      "discriminator": [
        88,
        0,
        193,
        113,
        213,
        210,
        191,
        140
      ]
    }
  ],
  "types": [
    {
      "name": "accountShape",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "i32"
          }
        ]
      }
    }
  ]
};
