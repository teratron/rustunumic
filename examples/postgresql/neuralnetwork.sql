-- Database: neuralnetwork

CREATE DATABASE neuralnetwork
    WITH
    OWNER = postgres
    ENCODING = 'UTF8';

-- Table: axon

CREATE TABLE "axon"
(
    "id"               serial4,
    "weight"           float8 NOT NULL DEFAULT 0,
    "incoming_cell_id" int4   NOT NULL,
    "outgoing_cell_id" int4   NOT NULL,
    PRIMARY KEY ("id")
);

ALTER TABLE "axon"
    ADD CONSTRAINT "incoming_cell_id_fkey" FOREIGN KEY ("incoming_cell_id") REFERENCES "cell" ("id");
ALTER TABLE "axon"
    ADD CONSTRAINT "outgoing_cell_id_fkey" FOREIGN KEY ("outgoing_cell_id") REFERENCES "cell" ("id");

-- Table: cell

CREATE TABLE "cell"
(
    "id"    serial4,
    "value" float8 NOT NULL DEFAULT 0,
    CONSTRAINT "_copy_4" PRIMARY KEY ("id")
);

-- Table: hidden_cell

CREATE TABLE "hidden_cell"
(
    "id"               serial4,
    "value"            float8 NOT NULL DEFAULT 0,
    "miss"             float8 NOT NULL DEFAULT 0,
    "incoming_axon_id" int4,
    "outgoing_axon_id" int4,
    CONSTRAINT "_copy_1" PRIMARY KEY ("id")
);

ALTER TABLE "hidden_cell"
    ADD CONSTRAINT "incoming_axon_id_fkey" FOREIGN KEY ("incoming_axon_id") REFERENCES "axon" ("id");
ALTER TABLE "hidden_cell"
    ADD CONSTRAINT "outgoing_axon_id_fkey" FOREIGN KEY ("outgoing_axon_id") REFERENCES "axon" ("id");

-- Table: input_cell

CREATE TABLE "input_cell"
(
    "id" serial4,
    PRIMARY KEY ("id")
);

-- Table: output_cell

CREATE TABLE "output_cell"
(
    "id"               serial4,
    "value"            float8 NOT NULL DEFAULT 0,
    "miss"             float8 NOT NULL DEFAULT 0,
    "target"           float8 NOT NULL,
    "incoming_axon_id" int4,
    CONSTRAINT "_copy_1_copy_1" PRIMARY KEY ("id")
);

ALTER TABLE "output_cell"
    ADD CONSTRAINT "incoming_axon_id_fkey" FOREIGN KEY ("incoming_axon_id") REFERENCES "axon" ("id");


