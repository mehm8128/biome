/* should not generate diagnostics */
export type * from "foo";
export type * as bar from "foo";
export type { foo } from "foo";
export type { baz, qux } from "foobar";
export type { moduleType as moduleType1 } from "module1";
export type { default as moduleType2 } from "module2";

export const A = 0;
