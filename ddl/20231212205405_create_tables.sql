--
--
--
CREATE TABLE task.tasks
{
    "id"     SERIAL NOT NULL,
    "note"   VARCHAR NOT NULL,
    "done"   BOOLEAN    NULL,

    PRIMARY KEY ("id")
}


