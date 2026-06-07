-- Redefine Member so system admin accounts do not need a card number or expiry.
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;

CREATE TABLE "new_Member" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "member_number" TEXT,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "joined_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expiry_date" DATETIME,
    "password_changed" BOOLEAN NOT NULL DEFAULT false,
    "must_setup" BOOLEAN NOT NULL DEFAULT false
);

INSERT INTO "new_Member" ("expiry_date", "first_name", "id", "joined_at", "last_name", "member_number", "must_setup", "password", "password_changed", "username")
SELECT "expiry_date", "first_name", "id", "joined_at", "last_name", "member_number", "must_setup", "password", "password_changed", "username"
FROM "Member";

DROP TABLE "Member";
ALTER TABLE "new_Member" RENAME TO "Member";

UPDATE "Member"
SET "member_number" = NULL,
    "expiry_date" = NULL
WHERE "id" IN (
    SELECT "memberId"
    FROM "UserRole"
    WHERE "role" = 'admin'
);

CREATE UNIQUE INDEX "Member_member_number_key" ON "Member"("member_number");
CREATE UNIQUE INDEX "Member_username_key" ON "Member"("username");

PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
