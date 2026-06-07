CREATE INDEX "Member_last_name_first_name_idx" ON "Member"("last_name", "first_name");
CREATE INDEX "Member_expiry_date_idx" ON "Member"("expiry_date");
CREATE INDEX "Attendance_check_in_day_idx" ON "Attendance"("check_in_day");
CREATE INDEX "Attendance_member_number_idx" ON "Attendance"("member_number");
