```
{
	"variables": VARIABLES_DICT
}

VARIABLES_DICT = 
{
	NAME: {
		NAME?: string 
		TYPE: string
		KEY?: {
			TYPE: string,
			OFFSET: number, 
			SLOT: number,
			SIZE: number,
			PATH: string
		},
		VALUES?: VARIABLES_DICT,
		OFFSET: number, 
		SLOT: number,
		SIZE: number,
		PATH: string
	}	
}
```