-- Store the recovery question in clear text so it can be shown during password recovery.
-- The answer remains stored only as a salted hash in recovery_phrase_hash.
ALTER TABLE "Member" ADD COLUMN "recovery_question" TEXT;
