#!/usr/bin/env -S  node --es-module-specifier-resolution=node --trace-uncaught --experimental-fetch --expose-gc --unhandled-rejections=strict 

import { readFileSync, writeFileSync } from "fs";
import WEBP from ".";
const { svgWebp } = WEBP;

const img = readFileSync("demo.svg");
console.log(await svgWebp(img, 10));
writeFileSync("demo.webp", Buffer.from(await svgWebp(img, 10)));
//console.log();
