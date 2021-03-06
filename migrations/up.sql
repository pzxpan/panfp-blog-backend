CREATE TABLE public.article
(
    article_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    user_id integer NOT NULL,
    title text COLLATE pg_catalog."default" NOT NULL,
    path text COLLATE pg_catalog."default",
    view_count integer DEFAULT 0,
    comment_count integer DEFAULT 0,
    like_count integer DEFAULT 0,
    date timestamp with time zone DEFAULT now(),
    intro text COLLATE pg_catalog."default" NOT NULL,
    content_html text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT article_pkey PRIMARY KEY (article_id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public."user" (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.article
    OWNER to test;

CREATE TABLE public.article_category
(
    article_id integer NOT NULL,
    category_id integer NOT NULL,
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 10000000 CACHE 1 ),
    CONSTRAINT article_category_pkey PRIMARY KEY (id),
    CONSTRAINT article_id FOREIGN KEY (article_id)
        REFERENCES public.article (article_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT category_id FOREIGN KEY (category_id)
        REFERENCES public.category (category_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.article_category
    OWNER to test;

CREATE TABLE public.article_label
(
    article_id integer NOT NULL,
    label_id integer NOT NULL,
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    CONSTRAINT article_label_pkey PRIMARY KEY (id),
    CONSTRAINT article_id FOREIGN KEY (article_id)
        REFERENCES public.article (article_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT label_id FOREIGN KEY (label_id)
        REFERENCES public.label (label_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.article_label
    OWNER to test;

CREATE TABLE public.article_like
(
    id integer NOT NULL GENERATED BY DEFAULT AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    user_id integer,
    article_id integer,
    CONSTRAINT article_like_pkey PRIMARY KEY (id),
    CONSTRAINT article_id FOREIGN KEY (article_id)
        REFERENCES public.article (article_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public."user" (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.article_like
    OWNER to test;


CREATE TABLE public.category
(
    category_id integer NOT NULL,
    name text COLLATE pg_catalog."default",
    category_alias text COLLATE pg_catalog."default",
    description text COLLATE pg_catalog."default",
    parent_id integer,
    CONSTRAINT category_pkey PRIMARY KEY (category_id)
)

TABLESPACE pg_default;

ALTER TABLE public.category
    OWNER to test;


CREATE TABLE public.comment
(
    comment_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    user_id integer NOT NULL,
    article_id integer NOT NULL,
    content text COLLATE pg_catalog."default",
    date timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT comment_pkey PRIMARY KEY (comment_id),
    CONSTRAINT article_id FOREIGN KEY (comment_id)
        REFERENCES public.comment (comment_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.comment
    OWNER to test;


CREATE TABLE public.hot_category
(
    category_id integer,
    name text COLLATE pg_catalog."default",
    description text COLLATE pg_catalog."default",
    category_alias text COLLATE pg_catalog."default",
    parent_id integer,
    hot_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    CONSTRAINT hot_category_pkey PRIMARY KEY (hot_id),
    CONSTRAINT category_id FOREIGN KEY (category_id)
        REFERENCES public.category (category_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.hot_category
    OWNER to test;


CREATE TABLE public.label
(
    label_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    name text COLLATE pg_catalog."default",
    label_alias text COLLATE pg_catalog."default",
    description text COLLATE pg_catalog."default",
    CONSTRAINT label_pkey PRIMARY KEY (label_id)
)

TABLESPACE pg_default;

ALTER TABLE public.label
    OWNER to test;



CREATE TABLE public."user"
(
    user_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 100000000 CACHE 1 ),
    password text COLLATE pg_catalog."default",
    email text COLLATE pg_catalog."default",
    register_time timestamp with time zone NOT NULL DEFAULT now(),
    nick_name text COLLATE pg_catalog."default",
    profession text COLLATE pg_catalog."default",
    level integer,
    avatar text COLLATE pg_catalog."default",
    expire timestamp with time zone DEFAULT (now() + '30 days'::interval),
    login_session text COLLATE pg_catalog."default",
    CONSTRAINT user_pkey PRIMARY KEY (user_id)
)

TABLESPACE pg_default;

ALTER TABLE public."user"
    OWNER to test;
    

-- Table: public.image

-- DROP TABLE public.image;

CREATE TABLE public.image
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 1000000000 CACHE 1 ),
    path text COLLATE pg_catalog."default",
    user_id integer,
    source_name text COLLATE pg_catalog."default",
    create_time timestamp with time zone DEFAULT now(),
    CONSTRAINT image_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public."user" (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.image
    OWNER to test;


-- FUNCTION: public.insert_article_detail(integer, text, text, text, integer, integer[])

-- DROP FUNCTION public.insert_article_detail(integer, text, text, text, integer, integer[]);

CREATE OR REPLACE FUNCTION public.insert_article_detail(
	u_id integer,
	a_title text,
	a_intro text,
	a_content_html text,
	a_category integer,
	a_labels integer[])
    RETURNS integer
    LANGUAGE 'plpgsql'

    COST 100
    VOLATILE

AS $BODY$DECLARE
a_id integer;
i integer;
a_label_id integer;
lens integer;
BEGIN
insert into public.article (user_id, title, intro,content_html) values (u_id,a_title,a_intro,a_content_html);
a_id = (select MAX(article_id) from public.article);
lens = array_length(a_labels,1);
for i in 1..lens loop
   a_label_id = a_labels[i];
   INSERT INTO public.article_label (article_id, label_id) VALUES (a_id,a_label_id);
end loop;
Insert into public.article_category (article_id,category_id) VALUES(a_id,a_category);
RETURN a_id;
END;$BODY$;

ALTER FUNCTION public.insert_article_detail(integer, text, text, text, integer, integer[])
    OWNER TO test;

-- FUNCTION: public.update_article_detail(integer, integer, text, text, text, integer, integer[])

-- DROP FUNCTION public.update_article_detail(integer, integer, text, text, text, integer, integer[]);

CREATE OR REPLACE FUNCTION public.update_article_detail(
	a_id integer,
	u_id integer,
	a_title text,
	a_intro text,
	a_content_html text,
	a_category integer,
	a_labels integer[])
    RETURNS integer
    LANGUAGE 'plpgsql'

    COST 100
    VOLATILE

AS $BODY$DECLARE
i integer;
a_label_id integer;
lens integer;
BEGIN
update public.article set title = a_title, intro = a_intro,content_html=a_content_html  where article_id = a_id and user_id = u_id;
delete from public.article_label where article_id = a_id;
lens = array_length(a_labels,1);
for i in 1..lens loop
   a_label_id = a_labels[i];
   INSERT INTO public.article_label (article_id, label_id) VALUES (a_id,a_label_id);
end loop;
update public.article_category set category_id=a_category where article_id = a_id;
RETURN a_id;
END;$BODY$;

ALTER FUNCTION public.update_article_detail(integer, integer, text, text, text, integer, integer[])
    OWNER TO test;

COMMENT ON FUNCTION public.update_article_detail(integer, integer, text, text, text, integer, integer[])
    IS '修改文章';
