CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE USER_ROLE AS ENUM ('USER', 'MANAGER', 'ROOT');
CREATE TYPE GENDER AS ENUM ('MALE', 'FEMALE');
CREATE TYPE MEDIA_TYPE AS ENUM ('AUDIO', 'VIDEO', 'PHOTO');
CREATE TYPE WHO_CAN AS ENUM ('FOLLOWED', 'FOLLOWERS');
CREATE TYPE FOLLOW_REQUEST_STATUS AS ENUM ('PENDING', 'APPROVED', 'REJECTED');

CREATE TABLE "users"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    gender GENDER NOT NULL,
    role USER_ROLE NOT NULL,
    bio VARCHAR(255) DEFAULT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    user_profile_image_id UUID DEFAULT NULL,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    activated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    verified_by UUID DEFAULT NULL
);

-- Create a trigger function to update users updated_at on every update
CREATE OR REPLACE FUNCTION update_users_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the update_users_updated_at function on every users update
CREATE TRIGGER trigger_update_users_updated_at BEFORE
UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_users_updated_at();

ALTER TABLE "users" ADD FOREIGN KEY (verified_by) REFERENCES users(id) ON DELETE RESTRICT;

CREATE TABLE "user_confirmation_tokens"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expire_at TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE "password_reset_tokens"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expire_at TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE "user_profile_images"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID NOT NULL,
    image_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

ALTER TABLE "users" ADD FOREIGN KEY (user_profile_image_id) REFERENCES user_profile_images(id) ON DELETE RESTRICT;

CREATE TABLE "hesses"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID NOT NULL,
    content TEXT DEFAULT NULL,
    parent_hess_id UUID DEFAULT NULL,

    -- If null, it means it follows the default user preferences
    -- If empty array, it means no one (except who's mentioned)
    who_can_reply WHO_CAN[] DEFAULT NULL,
    who_can_like WHO_CAN[] DEFAULT NULL,

    -- If null, it means it follows the default user preferences
    -- If empty array, it means no one (except who's mentioned)
    who_can_watch_replies WHO_CAN[] DEFAULT NULL,
    who_can_watch_likes WHO_CAN[] DEFAULT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (parent_hess_id) REFERENCES hesses(id) ON DELETE RESTRICT
);

-- Create a trigger function to update hesses updated_at on every update
CREATE OR REPLACE FUNCTION update_hesses_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the update_hesses_updated_at function on every hesses update
CREATE TRIGGER trigger_update_hesses_updated_at BEFORE
UPDATE ON hesses FOR EACH ROW EXECUTE FUNCTION update_hesses_updated_at();

CREATE TABLE "hess_mentions" (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    hess_id UUID NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (hess_id) REFERENCES hesses(id) ON DELETE RESTRICT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE "hess_media"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    hess_id UUID NOT NULL,
    media_type MEDIA_TYPE NOT NULL,
    media_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

CREATE TABLE "likes"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    hess_id UUID NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (hess_id) REFERENCES hesses(id) ON DELETE RESTRICT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE "user_privacy_preferences" (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,

    is_private_profile BOOLEAN NOT NULL DEFAULT FALSE,

    -- If null means pubic, if empty array means no one (except who's mentioned)
    who_can_reply WHO_CAN[] DEFAULT NULL,
    who_can_like WHO_CAN[] DEFAULT NULL,
    who_can_mention_me WHO_CAN[] DEFAULT NULL,

    -- If null means pubic, if empty array means no one (except who's mentioned)
    who_can_watch_new_hesses WHO_CAN[] DEFAULT NULL,
    who_can_watch_replies WHO_CAN[] DEFAULT NULL,
    who_can_watch_follows WHO_CAN[] DEFAULT NULL,
    who_can_watch_likes WHO_CAN[] DEFAULT NULL,

    -- its `created_at` is the same as user's `created_at`
    -- created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    -- its `deleted_at` is the same as user's `deleted_at`
    -- deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT
);

-- Create a trigger function to update user privacy preferences updated_at on every update
CREATE OR REPLACE FUNCTION update_user_privacy_preferences_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the update_user_privacy_preferences_updated_at function on every user privacy preferences update
CREATE TRIGGER trigger_update_user_privacy_preferences_updated_at BEFORE
UPDATE ON user_privacy_preferences FOR EACH ROW EXECUTE FUNCTION update_user_privacy_preferences_updated_at();

-- Create a trigger function to insert default privacy preferences for new users
CREATE OR REPLACE FUNCTION insert_default_privacy_preferences()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_privacy_preferences (user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the insert_default_privacy_preferences function on every user insert
CREATE TRIGGER after_insert_users
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION insert_default_privacy_preferences();


CREATE TABLE "followers"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,

    follower_id UUID NOT NULL,
    followed_id UUID NOT NULL,

    watch_new_hesses BOOLEAN NOT NULL,
    watch_replies BOOLEAN NOT NULL,
    watch_follows BOOLEAN NOT NULL,
    watch_likes BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (followed_id) REFERENCES users(id) ON DELETE RESTRICT
);

-- Create a trigger function to update followers updated_at on every update
CREATE OR REPLACE FUNCTION update_followers_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the update_followers_updated_at function on every follower update
CREATE TRIGGER trigger_update_followers_updated_at BEFORE
UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_followers_updated_at();


CREATE TABLE "follow_requests"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,

    requester_id UUID NOT NULL,
    requested_id UUID NOT NULL,

    watch_new_hesses BOOLEAN NOT NULL DEFAULT TRUE,
    watch_replies BOOLEAN NOT NULL DEFAULT TRUE,
    watch_follows BOOLEAN NOT NULL DEFAULT TRUE,
    watch_likes BOOLEAN NOT NULL DEFAULT FALSE,

    status FOLLOW_REQUEST_STATUS NOT NULL DEFAULT 'PENDING',

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    FOREIGN KEY (requested_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (requester_id) REFERENCES users(id) ON DELETE RESTRICT
);

-- Create a trigger function to update follow request updated_at on every update
CREATE OR REPLACE FUNCTION update_follow_requests_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the update_follow_requests_updated_at function on every follow_requests update
CREATE TRIGGER trigger_update_follow_requests_updated_at BEFORE
UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_follow_requests_updated_at();

CREATE TABLE "blocked_users"
(
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    blocker_id UUID NOT NULL,
    blocked_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (blocker_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (blocked_id) REFERENCES users(id) ON DELETE RESTRICT
);