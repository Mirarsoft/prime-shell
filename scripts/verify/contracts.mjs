import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const schemas = path.join(root, "packages/app-contracts/schemas");
const fixtures = path.join(root, "packages/app-contracts/fixtures");

const readJson = (file) =>
  JSON.parse(fs.readFileSync(path.join(root, file), "utf8"));

const schemaFiles = [
  "packages/app-contracts/schemas/handshake.schema.json",
  "packages/app-contracts/schemas/envelope.schema.json",
  "packages/app-contracts/schemas/errors.schema.json",
  "packages/app-contracts/schemas/task-events.schema.json",
  "packages/app-contracts/schemas/operations/spike.echo.schema.json",
  "packages/app-contracts/schemas/operations/spike.count.schema.json",
  "packages/app-contracts/schemas/operations/spike.crash.schema.json",
  "packages/app-contracts/schemas/operations/spike.hang.schema.json",
  "packages/app-contracts/schemas/operations/spike.largeRejected.schema.json",
];
const allSchemas = schemaFiles.map(readJson);
const schemaByTitle = new Map(
  allSchemas.map((schema) => [schema.title, schema]),
);
const handshake = schemaByTitle.get("Prime Shell Backend Hello");
const envelope = schemaByTitle.get("Prime Shell Request and Response Envelopes");
const taskEvents = schemaByTitle.get("Prime Shell Task and Backend State");
if (!handshake || !envelope || !taskEvents) {
  throw new Error("required contract schemas are missing");
}

const ajv = new Ajv2020({ allErrors: true, strict: true });
for (const schema of allSchemas) {
  ajv.addSchema(schema);
}

const validateHandshake = ajv.getSchema(handshake.$id);
const validateEnvelope = ajv.getSchema(envelope.$id);
const validateBackendStatus = ajv.getSchema(
  `${taskEvents.$id}#/$defs/backendStatus`,
);
if (!validateHandshake || !validateEnvelope || !validateBackendStatus) {
  throw new Error("contract validators were not compiled");
}

const validCases = [
  [validateHandshake, "valid/hello.json"],
  [validateEnvelope, "valid/echo-request.json"],
  [validateEnvelope, "valid/echo-result.json"],
  [validateEnvelope, "valid/count-request.json"],
  [validateEnvelope, "valid/count-accepted.json"],
  [validateEnvelope, "valid/count-progress.json"],
  [validateEnvelope, "valid/count-cancel.json"],
  [validateEnvelope, "valid/count-cancel-ack.json"],
  [validateEnvelope, "valid/count-cancelled.json"],
  [validateEnvelope, "valid/crash-request.json"],
  [validateEnvelope, "valid/hang-request.json"],
  [validateEnvelope, "valid/large-rejected-request.json"],
  [validateEnvelope, "valid/large-rejected-error.json"],
  [validateBackendStatus, "valid/backend-status.json"],
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
  "invalid/count-out-of-range.json",
  "invalid/task-sequence-zero.json",
  "invalid/cancel-extra-property.json",
  "invalid/large-below-threshold.json",
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
    schemas: schemaFiles.length,
    validFixtures: validCases.length,
    invalidFixtures: 7,
  }),
);
