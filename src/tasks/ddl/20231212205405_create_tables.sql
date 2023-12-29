--
--
--


CREATE TABLE tasks.tasks
(
    "id"   SERIAL  NOT NULL,
    "note" VARCHAR NOT NULL,
    "done" BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY ("id")
);
