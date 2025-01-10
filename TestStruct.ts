interface TestStruct {
  Integer: number;
  idx: number;
  float: number;
  boolean: boolean;
  character: string;
  string: string;
  option: string | null;
  vector: number[];
  tuple: [string, number, boolean];
  complex: [string | null, number][];
  custom_enum: CustomEnum;
}
