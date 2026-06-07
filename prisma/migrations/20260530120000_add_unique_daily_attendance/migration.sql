-- Store the local check-in day separately so the database can prevent
-- duplicate attendance rows for the same member on the same day.
ALTER TABLE "Attendance" ADD COLUMN "check_in_day" TEXT NOT NULL DEFAULT '';

UPDATE "Attendance"
SET "check_in_day" = COALESCE(
    date("check_in_time"),
    date("check_in_time", 'unixepoch'),
    date("check_in_time" / 1000, 'unixepoch'),
    substr(CAST("check_in_time" AS TEXT), 1, 10)
)
WHERE "check_in_day" = '';

DELETE FROM "Attendance"
WHERE rowid NOT IN (
    SELECT MIN(rowid)
    FROM "Attendance"
    GROUP BY "member_id", "check_in_day"
);

CREATE UNIQUE INDEX "Attendance_member_id_check_in_day_key" ON "Attendance"("member_id", "check_in_day");
