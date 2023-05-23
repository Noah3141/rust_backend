-- Table: Accounts_Scheme.Accounts

-- DROP TABLE IF EXISTS "Accounts_Scheme"."Accounts";

CREATE TABLE IF NOT EXISTS "Accounts_Scheme"."Accounts"
(
    account_id integer NOT NULL DEFAULT nextval('"Accounts_Scheme"."Accounts_id_seq"'::regclass),
    username character varying(25) COLLATE pg_catalog."default" NOT NULL,
    hash character varying(20) COLLATE pg_catalog."default" NOT NULL,
    salt character varying(15) COLLATE pg_catalog."default" NOT NULL,
    account_start_date timestamp without time zone NOT NULL,
    account_deleted boolean NOT NULL,
    membership_status boolean NOT NULL,
    active_membership_start_date timestamp without time zone,
    active_membership_end_date timestamp without time zone,
    first_membership_date timestamp without time zone,
    pro_status boolean NOT NULL,
    CONSTRAINT "Accounts_pkey" PRIMARY KEY (account_id)
    CONSTRAINT "Accounts_username_key" UNIQUE (username)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS "Accounts_Scheme"."Accounts"
    OWNER to postgres;