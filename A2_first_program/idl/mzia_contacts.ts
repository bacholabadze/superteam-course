/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/mzia_contacts.json`.
 */
export type MziaContacts = {
  "address": "5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H",
  "metadata": {
    "name": "mziaContacts",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addContact",
      "docs": [
        "Save a trusted recipient under a short name (e.g. \"deda\")."
      ],
      "discriminator": [
        184,
        47,
        126,
        40,
        45,
        107,
        85,
        172
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "contact",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  97,
                  99,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "arg",
                "path": "name"
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "address",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "removeContact",
      "docs": [
        "Delete a saved contact and refund its rent to the owner."
      ],
      "discriminator": [
        2,
        132,
        185,
        93,
        96,
        119,
        186,
        89
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "contact"
          ]
        },
        {
          "name": "contact",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  97,
                  99,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "contact.name",
                "account": "contact"
              }
            ]
          }
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "contact",
      "discriminator": [
        227,
        113,
        227,
        39,
        193,
        116,
        172,
        45
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidName",
      "msg": "Contact name must be 1-32 bytes"
    },
    {
      "code": 6001,
      "name": "selfContact",
      "msg": "A contact cannot point to its own owner"
    }
  ],
  "types": [
    {
      "name": "contact",
      "docs": [
        "A trusted recipient saved by a wallet owner, e.g. \"deda\" -> Mom's address.",
        "One PDA per (owner, name), so a name is unique within an owner's book."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "contactSeed",
      "type": "bytes",
      "value": "[99, 111, 110, 116, 97, 99, 116]"
    }
  ]
};
