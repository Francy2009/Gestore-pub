-- CreateTable
CREATE TABLE "Member" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "member_number" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "joined_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expiry_date" DATETIME NOT NULL,
    "password_changed" BOOLEAN NOT NULL DEFAULT false,
    "must_setup" BOOLEAN NOT NULL DEFAULT false
);

-- CreateTable
CREATE TABLE "UserRole" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "role" TEXT NOT NULL,
    "memberId" TEXT NOT NULL,
    CONSTRAINT "UserRole_memberId_fkey" FOREIGN KEY ("memberId") REFERENCES "Member" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Attendance" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "member_id" TEXT NOT NULL,
    "check_in_time" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "Attendance_member_id_fkey" FOREIGN KEY ("member_id") REFERENCES "Member" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "Member_member_number_key" ON "Member"("member_number");

-- CreateIndex
CREATE UNIQUE INDEX "Member_username_key" ON "Member"("username");

-- CreateIndex
CREATE UNIQUE INDEX "UserRole_memberId_key" ON "UserRole"("memberId");
