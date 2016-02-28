use libc::c_int;

pub const SQLITE_INTEGER: c_int = 1;
pub const SQLITE_FLOAT  : c_int = 2;
pub const SQLITE_TEXT   : c_int = 3;
pub const SQLITE_BLOB   : c_int = 4;
pub const SQLITE_NULL   : c_int = 5;

pub const SQLITE_OK        : c_int =   0;
pub const SQLITE_ERROR     : c_int =   1;
pub const SQLITE_INTERNAL  : c_int =   2;
pub const SQLITE_PERM      : c_int =   3;
pub const SQLITE_ABORT     : c_int =   4;
pub const SQLITE_BUSY      : c_int =   5;
pub const SQLITE_LOCKED    : c_int =   6;
pub const SQLITE_NOMEM     : c_int =   7;
pub const SQLITE_READONLY  : c_int =   8;
pub const SQLITE_INTERRUPT : c_int =   9;
pub const SQLITE_IOERR     : c_int =  10;
pub const SQLITE_CORRUPT   : c_int =  11;
pub const SQLITE_NOTFOUND  : c_int =  12;
pub const SQLITE_FULL      : c_int =  13;
pub const SQLITE_CANTOPEN  : c_int =  14;
pub const SQLITE_PROTOCOL  : c_int =  15;
pub const SQLITE_EMPTY     : c_int =  16;
pub const SQLITE_SCHEMA    : c_int =  17;
pub const SQLITE_TOOBIG    : c_int =  18;
pub const SQLITE_CONSTRAINT: c_int =  19;
pub const SQLITE_MISMATCH  : c_int =  20;
pub const SQLITE_MISUSE    : c_int =  21;
pub const SQLITE_NOLFS     : c_int =  22;
pub const SQLITE_AUTH      : c_int =  23;
pub const SQLITE_FORMAT    : c_int =  24;
pub const SQLITE_RANGE     : c_int =  25;
pub const SQLITE_NOTADB    : c_int =  26;
pub const SQLITE_NOTICE    : c_int =  27;
pub const SQLITE_WARNING   : c_int =  28;
pub const SQLITE_ROW       : c_int = 100;
pub const SQLITE_DONE      : c_int = 101;

pub const SQLITE_IOERR_READ             : c_int = (SQLITE_IOERR      | ( 1 << 8));
pub const SQLITE_IOERR_SHORT_READ       : c_int = (SQLITE_IOERR      | ( 2 << 8));
pub const SQLITE_IOERR_WRITE            : c_int = (SQLITE_IOERR      | ( 3 << 8));
pub const SQLITE_IOERR_FSYNC            : c_int = (SQLITE_IOERR      | ( 4 << 8));
pub const SQLITE_IOERR_DIR_FSYNC        : c_int = (SQLITE_IOERR      | ( 5 << 8));
pub const SQLITE_IOERR_TRUNCATE         : c_int = (SQLITE_IOERR      | ( 6 << 8));
pub const SQLITE_IOERR_FSTAT            : c_int = (SQLITE_IOERR      | ( 7 << 8));
pub const SQLITE_IOERR_UNLOCK           : c_int = (SQLITE_IOERR      | ( 8 << 8));
pub const SQLITE_IOERR_RDLOCK           : c_int = (SQLITE_IOERR      | ( 9 << 8));
pub const SQLITE_IOERR_DELETE           : c_int = (SQLITE_IOERR      | (10 << 8));
pub const SQLITE_IOERR_BLOCKED          : c_int = (SQLITE_IOERR      | (11 << 8));
pub const SQLITE_IOERR_NOMEM            : c_int = (SQLITE_IOERR      | (12 << 8));
pub const SQLITE_IOERR_ACCESS           : c_int = (SQLITE_IOERR      | (13 << 8));
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: c_int = (SQLITE_IOERR      | (14 << 8));
pub const SQLITE_IOERR_LOCK             : c_int = (SQLITE_IOERR      | (15 << 8));
pub const SQLITE_IOERR_CLOSE            : c_int = (SQLITE_IOERR      | (16 << 8));
pub const SQLITE_IOERR_DIR_CLOSE        : c_int = (SQLITE_IOERR      | (17 << 8));
pub const SQLITE_IOERR_SHMOPEN          : c_int = (SQLITE_IOERR      | (18 << 8));
pub const SQLITE_IOERR_SHMSIZE          : c_int = (SQLITE_IOERR      | (19 << 8));
pub const SQLITE_IOERR_SHMLOCK          : c_int = (SQLITE_IOERR      | (20 << 8));
pub const SQLITE_IOERR_SHMMAP           : c_int = (SQLITE_IOERR      | (21 << 8));
pub const SQLITE_IOERR_SEEK             : c_int = (SQLITE_IOERR      | (22 << 8));
pub const SQLITE_IOERR_DELETE_NOENT     : c_int = (SQLITE_IOERR      | (23 << 8));
pub const SQLITE_IOERR_MMAP             : c_int = (SQLITE_IOERR      | (24 << 8));
pub const SQLITE_IOERR_GETTEMPPATH      : c_int = (SQLITE_IOERR      | (25 << 8));
pub const SQLITE_IOERR_CONVPATH         : c_int = (SQLITE_IOERR      | (26 << 8));
pub const SQLITE_LOCKED_SHAREDCACHE     : c_int = (SQLITE_LOCKED     | ( 1 << 8));
pub const SQLITE_BUSY_RECOVERY          : c_int = (SQLITE_BUSY       | ( 1 << 8));
pub const SQLITE_BUSY_SNAPSHOT          : c_int = (SQLITE_BUSY       | ( 2 << 8));
pub const SQLITE_CANTOPEN_NOTEMPDIR     : c_int = (SQLITE_CANTOPEN   | ( 1 << 8));
pub const SQLITE_CANTOPEN_ISDIR         : c_int = (SQLITE_CANTOPEN   | ( 2 << 8));
pub const SQLITE_CANTOPEN_FULLPATH      : c_int = (SQLITE_CANTOPEN   | ( 3 << 8));
pub const SQLITE_CANTOPEN_CONVPATH      : c_int = (SQLITE_CANTOPEN   | ( 4 << 8));
pub const SQLITE_CORRUPT_VTAB           : c_int = (SQLITE_CORRUPT    | ( 1 << 8));
pub const SQLITE_READONLY_RECOVERY      : c_int = (SQLITE_READONLY   | ( 1 << 8));
pub const SQLITE_READONLY_CANTLOCK      : c_int = (SQLITE_READONLY   | ( 2 << 8));
pub const SQLITE_READONLY_ROLLBACK      : c_int = (SQLITE_READONLY   | ( 3 << 8));
pub const SQLITE_READONLY_DBMOVED       : c_int = (SQLITE_READONLY   | ( 4 << 8));
pub const SQLITE_ABORT_ROLLBACK         : c_int = (SQLITE_ABORT      | ( 2 << 8));
pub const SQLITE_CONSTRAINT_CHECK       : c_int = (SQLITE_CONSTRAINT | ( 1 << 8));
pub const SQLITE_CONSTRAINT_COMMITHOOK  : c_int = (SQLITE_CONSTRAINT | ( 2 << 8));
pub const SQLITE_CONSTRAINT_FOREIGNKEY  : c_int = (SQLITE_CONSTRAINT | ( 3 << 8));
pub const SQLITE_CONSTRAINT_FUNCTION    : c_int = (SQLITE_CONSTRAINT | ( 4 << 8));
pub const SQLITE_CONSTRAINT_NOTNULL     : c_int = (SQLITE_CONSTRAINT | ( 5 << 8));
pub const SQLITE_CONSTRAINT_PRIMARYKEY  : c_int = (SQLITE_CONSTRAINT | ( 6 << 8));
pub const SQLITE_CONSTRAINT_TRIGGER     : c_int = (SQLITE_CONSTRAINT | ( 7 << 8));
pub const SQLITE_CONSTRAINT_UNIQUE      : c_int = (SQLITE_CONSTRAINT | ( 8 << 8));
pub const SQLITE_CONSTRAINT_VTAB        : c_int = (SQLITE_CONSTRAINT | ( 9 << 8));
pub const SQLITE_CONSTRAINT_ROWID       : c_int = (SQLITE_CONSTRAINT | (10 << 8));
pub const SQLITE_NOTICE_RECOVER_WAL     : c_int = (SQLITE_NOTICE     | ( 1 << 8));
pub const SQLITE_NOTICE_RECOVER_ROLLBACK: c_int = (SQLITE_NOTICE     | ( 2 << 8));
pub const SQLITE_WARNING_AUTOINDEX      : c_int = (SQLITE_WARNING    | ( 1 << 8));
pub const SQLITE_AUTH_USER              : c_int = (SQLITE_AUTH       | ( 1 << 8));

pub const SQLITE_OPEN_READONLY      : c_int = 0x00000001;
pub const SQLITE_OPEN_READWRITE     : c_int = 0x00000002;
pub const SQLITE_OPEN_CREATE        : c_int = 0x00000004;
pub const SQLITE_OPEN_DELETEONCLOSE : c_int = 0x00000008;
pub const SQLITE_OPEN_EXCLUSIVE     : c_int = 0x00000010;
pub const SQLITE_OPEN_AUTOPROXY     : c_int = 0x00000020;
pub const SQLITE_OPEN_URI           : c_int = 0x00000040;
pub const SQLITE_OPEN_MEMORY        : c_int = 0x00000080;
pub const SQLITE_OPEN_MAIN_DB       : c_int = 0x00000100;
pub const SQLITE_OPEN_TEMP_DB       : c_int = 0x00000200;
pub const SQLITE_OPEN_TRANSIENT_DB  : c_int = 0x00000400;
pub const SQLITE_OPEN_MAIN_JOURNAL  : c_int = 0x00000800;
pub const SQLITE_OPEN_TEMP_JOURNAL  : c_int = 0x00001000;
pub const SQLITE_OPEN_SUBJOURNAL    : c_int = 0x00002000;
pub const SQLITE_OPEN_MASTER_JOURNAL: c_int = 0x00004000;
pub const SQLITE_OPEN_NOMUTEX       : c_int = 0x00008000;
pub const SQLITE_OPEN_FULLMUTEX     : c_int = 0x00010000;
pub const SQLITE_OPEN_SHAREDCACHE   : c_int = 0x00020000;
pub const SQLITE_OPEN_PRIVATECACHE  : c_int = 0x00040000;
pub const SQLITE_OPEN_WAL           : c_int = 0x00080000;
