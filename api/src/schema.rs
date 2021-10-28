table! {
    comment (cid) {
        cid -> Int4,
        uid -> Int8,
        pid -> Nullable<Int8>,
        status -> Nullable<Int2>,
        bundle -> Varchar,
        target_id -> Int8,
        subject -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        hostname -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
    }
}

table! {
    comment_body (cid) {
        cid -> Int8,
        body -> Nullable<Text>,
        body_format -> Varchar,
    }
}

table! {
    config (name) {
        name -> Varchar,
        data -> Varchar,
    }
}

table! {
    file (fid) {
        fid -> Int4,
        uid -> Int4,
        filename -> Varchar,
        uri -> Varchar,
        storage -> Varchar,
        mime -> Nullable<Varchar>,
        sie -> Nullable<Int8>,
        status -> Nullable<Int2>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    node (nid) {
        nid -> Int4,
        vid -> Varchar,
        uid -> Int4,
        bundle -> Varchar,
        title -> Varchar,
        deleted -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
    }
}

table! {
    node_body (nid) {
        nid -> Int4,
        summary -> Nullable<Text>,
        body -> Nullable<Text>,
        body_format -> Nullable<Varchar>,
    }
}

table! {
    node_category_map (nid, tid) {
        bundle -> Varchar,
        nid -> Int4,
        tid -> Int4,
    }
}

table! {
    node_comments_map (nid, cid) {
        bundle -> Varchar,
        nid -> Int4,
        cid -> Int8,
    }
}

table! {
    node_images_map (nid, fid) {
        bundle -> Varchar,
        nid -> Int4,
        fid -> Int4,
        weight -> Nullable<Int4>,
        alt -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
    }
}

table! {
    node_tags_map (nid, tid) {
        bundle -> Varchar,
        nid -> Int4,
        tid -> Int4,
    }
}

table! {
    taxonomy (tid) {
        tid -> Int4,
        vid -> Varchar,
        pid -> Nullable<Int4>,
        bundle -> Varchar,
        name -> Varchar,
        description -> Varchar,
        description_format -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
    }
}

table! {
    user_picture (uid, fid) {
        bundle -> Varchar,
        uid -> Int4,
        fid -> Int4,
        weight -> Nullable<Int4>,
        alt -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        width -> Nullable<Int8>,
        height -> Nullable<Int8>,
    }
}

table! {
    users (uid) {
        uid -> Int4,
        username -> Varchar,
        nickname -> Nullable<Varchar>,
        password -> Varchar,
        status -> Int2,
        email -> Varchar,
        admin -> Nullable<Bool>,
        intro -> Nullable<Varchar>,
        last_login_on -> Nullable<Timestamp>,
        salt -> Nullable<Varchar>,
        must_change_password -> Nullable<Bool>,
        password_changed_on -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    comment,
    comment_body,
    config,
    file,
    node,
    node_body,
    node_category_map,
    node_comments_map,
    node_images_map,
    node_tags_map,
    taxonomy,
    user_picture,
    users,
);
