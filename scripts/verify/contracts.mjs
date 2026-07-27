import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const schemas = path.join(root, "packages/app-contracts/schemas");
const fixtures = path.join(root, "packages/app-contracts/fixtures");

const readJson = (file) =>
  JSON.parse(fs.readFileSync(path.join(root, file), "utf8"));

const handshake = readJson(
  "packages/app-contracts/schemas/handshake.schema.json",
);
const envelope = readJson(
  "packages/app-contracts/schemas/envelope.schema.json",
);
const errors = readJson("packages/app-contracts/schemas/errors.schema.json");
const echo = readJson(
  "packages/app-contracts/schemas/operations/spike.echo.schema.json",
);

const ajv = new Ajv2020({ allErrors: true, strict: true });
for (const schema of [handshake, envelope, errors, echo]) {
  ajv.addSchema(schema);
}

const validateHandshake = ajv.getSchema(handshake.$id);
const validateEnvelope = ajv.getSchema(envelope.$id);
if (!validateHandshake || !validateEnvelope) {
  throw new Error("contract validators were not compiled");
}

const validCases = [
  [validateHandshake, "valid/hello.json"],
  [validateEnvelope, "valid/echo-request.json"],
  [validateEnvelope, "valid/echo-result.json"],
];
for (const [validate, name] of validCases) {
  const value = JSON.parse(fs.readFileSync(path.join(fixtures, name), "utf8"));
  if (!validate(value)) {
    throw new Error(`${name} should be valid: ${ajv.errorsText(validate.errors)}`);
  }
}

for (const name of [
  "invalid/unknown-operation.json",
  "invalid/echo-extra-property.json",
]) {
  const value = JSON.parse(fs.readFileSync(path.join(fixtures, name), "utf8"));
  if (validateEnvelope(value)) {
    throw new Error(`${name} should be invalid`);
  }
}

try {
  JSON.parse(
    fs.readFileSync(path.join(fixtures, "invalid/malformed.jsonl"), "utf8"),
  );
  throw new Error("malformed.jsonl should not parse");
} catch (error) {
  if (!(error instanceof SyntaxError)) {
    throw error;
  }
}

console.log(
  JSON.stringify({
    status: "passed",
    schemaDraft: "2020-12",
    schemas: fs.readdirSync(schemas).length + 1,
    validFixtures: validCases.length,
    invalidFixtures: 3,
  }),
);
