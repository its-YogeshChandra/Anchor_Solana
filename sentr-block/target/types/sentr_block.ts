/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/sentr_block.json`.
 */
export type SentrBlock = {
  "address": "DPan51KtNPYScdj8uDPAraoAXxiM2bdieRpnHSVZPrHT",
  "metadata": {
    "name": "sentrBlock",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
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
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "pdaAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  104,
                  101,
                  108,
                  108,
                  111,
                  95,
                  119,
                  111,
                  114,
                  108,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "customAccount",
      "discriminator": [
        99,
        151,
        198,
        27,
        164,
        9,
        6,
        138
      ]
    }
  ],
  "types": [
    {
      "name": "customAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bumpSeed",
            "type": "bytes"
          }
        ]
      }
    }
  ]
};
