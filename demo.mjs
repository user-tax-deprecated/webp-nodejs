#!/usr/bin/env -S  node --es-module-specifier-resolution=node --trace-uncaught --experimental-fetch --expose-gc --unhandled-rejections=strict 

import { readFileSync, writeFileSync } from "fs";
import WEBP from ".";
const { svgWebp } = WEBP;

const img = readFileSync("demo.svg");
writeFileSync("demo.webp", Buffer.from(svgWebp(img, 30)));
//console.log();
