-- Add a separate opaque QR token so public cards do not expose Member.id.
ALTER TABLE "Member" ADD COLUMN "qr_token" TEXT;

UPDATE "Member"
SET "qr_token" = lower(hex(randomblob(16)))
WHERE "qr_token" IS NULL
  AND "id" IN (
    SELECT "memberId"
    FROM "UserRole"
    WHERE "role" = 'user'
  );

CREATE UNIQUE INDEX "Member_qr_token_key" ON "Member"("qr_token");
