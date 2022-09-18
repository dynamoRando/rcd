# 2022-09-17
Hopefully soon planned changes:
- [X] Implement UPDATE from host to participant
- [X] Implement DELETE from host to participant (this is a hard delete)
- Implement 'soft' DELETE from host to participant
- Write logs to a rcd_log.db 
- Modify INSERT/UPDATE/DELETE from host to participant so that at the participant we check if 
    - the host has been banned
    - if we will notify the host of local changes done our side (there is no well defined infastructure currently to handle this at the participant)
        although we do have placeholder functions in the .proto for this