PRAGMA foreign_keys=OFF;

CREATE TABLE "new_Attendance" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "member_id" TEXT,
    "check_in_time" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "check_in_day" TEXT NOT NULL,
    "member_first_name" TEXT NOT NULL,
    "member_last_name" TEXT NOT NULL,
    "member_number" TEXT NOT NULL,
    "member_was_deleted" BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT "Attendance_member_id_fkey" FOREIGN KEY ("member_id") REFERENCES "Member" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);

INSERT INTO "new_Attendance" (
    "id",
    "member_id",
    "check_in_time",
    "check_in_day",
    "member_first_name",
    "member_last_name",
    "member_number",
    "member_was_deleted"
)
SELECT
    "Attendance"."id",
    "Attendance"."member_id",
    "Attendance"."check_in_time",
    "Attendance"."check_in_day",
    COALESCE("Member"."first_name", 'Socio'),
    COALESCE("Member"."last_name", 'eliminato'),
    COALESCE("Member"."member_number", 'N/D'),
    CASE WHEN "Member"."id" IS NULL THEN true ELSE false END
FROM "Attendance"
LEFT JOIN "Member" ON "Member"."id" = "Attendance"."member_id";

DROP TABLE "Attendance";
ALTER TABLE "new_Attendance" RENAME TO "Attendance";

CREATE UNIQUE INDEX "Attendance_member_id_check_in_day_key" ON "Attendance"("member_id", "check_in_day");

PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
