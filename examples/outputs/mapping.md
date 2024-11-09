```sol
mapping(uint8 => mapping(uint8 => address)) public home_map;
```

```
{
    "variables": {
      home_app: {
          NAME: home_map 
          TYPE: "mapping"
            KEY: {
                TYPE: "uint8",
                OFFSET: number, 
                SLOT: number,
                SIZE: number,
                ADDRESS (KECCAK): string,
                PATH: string
            },
          VALUES: [
                {
                    TYPE: "mapping"
                    KEY: {
                        TYPE: "uint8"
                        OFFSET: number, 
                        SLOT: number,
                        SIZE: number,
                        ADDRESS (KECCAK): string,
                        PATH: string
                    },
                    VALUES: [
                        {
                            TYPE: "addresse",
                            OFFSET: number, 
                            SLOT: number,
                            SIZE: number,
                            ADDRESS (KECCAK): string,
                            PATH: string
                        },
                    ]
                    OFFSET: number, 
                    SLOT: number,
                    SIZE: number,
                    ADDRESS (KECCAK): string,
                    PATH: string
                }
            ],
          OFFSET: number, 
          SLOT: number,
          SIZE: number,
          ADDRESS (KECCAK): string,
          PATH: string
      }
    }
}
```