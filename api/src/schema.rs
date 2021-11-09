table! {
    comment (cid) {
        cid -> Int4,
        uid -> Int8,
        pid -> Int8,
        status -> Int2,
        bundle -> Varchar,
        target_id -> Int8,
        subject -> Varchar,
        name -> Varchar,
        email -> Varchar,
        homepage -> Varchar,
        hostname -> Varchar,
        created_at -> Timestamp,
        created_by -> Int4,
        updated_at -> Timestamp,
        updated_by -> Int4,
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
        mime -> Varchar,
        sie -> Int8,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    node (nid) {
        nid -> Int4,
        vid -> Varchar,
        uid -> Int4,
        bundle -> Varchar,
        title -> Varchar,
        deleted -> Bool,
        published_at -> Int4,
        created_at -> Timestamp,
        created_by -> Int4,
        updated_at -> Timestamp,
        updated_by -> Int4,
    }
}

table! {
    node_body (nid) {
        nid -> Int4,
        summary -> Nullable<Text>,
        body -> Nullable<Text>,
        body_format -> Varchar,
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
        weight -> Int4,
        alt -> Varchar,
        title -> Varchar,
        width -> Int4,
        height -> Int4,
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
        pid -> Int4,
        bundle -> Varchar,
        name -> Varchar,
        description -> Varchar,
        description_format -> Varchar,
        weight -> Int4,
    }
}

table! {
    user_picture (uid, fid) {
        bundle -> Varchar,
        uid -> Int4,
        fid -> Int4,
        weight -> Int4,
        alt -> Varchar,
        title -> Varchar,
        width -> Int8,
        height -> Int8,
    }
}

table! {
    users (uid) {
        uid -> Int4,
        username -> Varchar,
        nickname -> Varchar,
        password -> Varchar,
        status -> Int2,
        email -> Varchar,
        admin -> Bool,
        intro -> Varchar,
        last_login_on -> Timestamp,
        salt -> Varchar,
        must_change_password -> Bool,
        password_changed_on -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
