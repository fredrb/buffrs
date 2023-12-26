// Copyright 2023 Helsing GmbH
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use atmosphere::prelude::*;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

/// A user of the registry
#[derive(Schema, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[table(schema = "public", name = "users")]
pub struct User {
    #[sql(pk)]
    pub id: i32,
    #[sql(unique)]
    pub handle: String,
    #[sql(timestamp = created)]
    //#[hook(insert = DateTime::now())]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp = updated)]
    //#[hook(update = DateTime::now())]
    pub updated_at: DateTime<Utc>,
    #[sql(timestamp = deleted)]
    //#[hook(delete = DateTime::now())]
    pub deleted_at: Option<DateTime<Utc>>,
}

/// A token that is identifying a user / used for authentication
#[derive(Schema, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[table(schema = "public", name = "user_tokens")]
pub struct UserToken {
    #[sql(pk)]
    pub hash: String,
    #[sql(fk -> User, rename = "user_id")]
    pub user: i32,
    #[sql(unique)]
    pub prefix: String,
    pub allow_publish: bool,
    pub allow_update: bool,
    pub allow_yank: bool,
    #[sql(timestamp = created)]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp = updated)]
    pub updated_at: DateTime<Utc>,
    #[sql(timestamp = deleted)]
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Buffrs Package Types
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "package_type", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum PackageType {
    /// A library package
    Library,
    /// An api package
    Api,
}

/// A package tracked by the registry
#[derive(Schema, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[table(schema = "public", name = "packages")]
pub struct Package {
    #[sql(pk)]
    pub id: i32,
    #[sql(unique)]
    pub name: String,
    #[sql(rename = "type")]
    pub ty: PackageType,
    #[sql(timestamp = created)]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp = updated)]
    pub updated_at: DateTime<Utc>,
}

/// A package owner
#[derive(Schema, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[table(schema = "public", name = "package_owners")]
pub struct PackageOwner {
    #[sql(pk)]
    pub id: i32,
    //#[sql(fk -> User, rename = "user_id")]
    #[sql(rename = "user_id")]
    pub user: i32,
    //#[sql(fk -> Package, rename = "package_id")]
    #[sql(rename = "package_id")]
    pub package: i32,
    //#[sql(fk -> User, rename = "invited_by")]
    pub invited_by: i32,
    #[sql(timestamp = created)]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp = updated)]
    pub updated_at: DateTime<Utc>,
    #[sql(timestamp = deleted)]
    pub deleted_at: Option<DateTime<Utc>>,
}

/// A package invite
#[derive(Schema, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[table(schema = "public", name = "package_invites")]
pub struct PackageInvite {
    #[sql(pk)]
    pub id: i32,
    #[sql(unique)]
    pub token: String,
    //#[sql(fk -> User, rename = "user_id")]
    #[sql(rename = "user_id")]
    pub user: i32,
    //#[sql(fk -> Package, rename = "package_id")]
    #[sql(rename = "package_id")]
    pub package: i32,
    //#[sql(fk -> User, rename = "invited_by")]
    pub invited_by: i32,
    #[sql(timestamp = created)]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp = updated)]
    pub updated_at: DateTime<Utc>,
    #[sql(timestamp = deleted)]
    pub deleted_at: Option<DateTime<Utc>>,
}