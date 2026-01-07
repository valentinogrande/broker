-- create database OurBroker;
-- use database OurBroker;

create table if not exists users (
  id bigint unsigned auto_increment primary key,
  
  username varchar(100) not null unique,
  password  varchar(255) not null,
  email varchar(255) not null unique,
  
  photo text,

  is_admin boolean default false
);

create table if not exists individuals (
  id bigint unsigned auto_increment primary key,
  
  user_id bigint unsigned not null,
  
  address varchar(255) not null,
  name varchar(255) not null,
  birth_date date not null,
  
  trading_experience enum ('none', 'basic', 'intermediate', 'advanced', 'professional') not null,
  pep bool not null default false, --person exposed politically
  annual_income enum ('<10k', '10k-25k', '25k-50k', '50k-100k', '100k-250k', '>250k') not null,
  transaction_amount enum ('<1k', '1k-5k', '5k-20k', '20k-50k', '>50k') not null,
  is_owner_beneficiary bool not null default false,
  source_of_funds text not null,

  created_at timestamp default current_timestamp,
  updated_at timestamp default current_timestamp on update current_timestamp,


  unique key ind_user (user_id),
  foreign key (user_id) references users(id) on delete cascade
);

create table if not exists companies (
  id bigint unsigned auto_increment primary key,
  
  user_id bigint unsigned not null,
  
  company_name varchar(255) not null,
  registration_number bigint unsigned not null,
  country char(2) not null,-- ISO 3166-1 
  
  status enum ('active', 'inactive') not null default 'active',
  
  company_type varchar(128) not null,
  nature_of_business varchar(255) not null,
  
  incorporation_date date not null,
  
  registered_address text not null,

  created_at timestamp default current_timestamp,
  updated_at timestamp default current_timestamp on update current_timestamp,

  unique key comp_user (user_id),
  foreign key (user_id) references users(id) on delete cascade
);

create table if not exists kyb_company_officers (
  id bigint unsigned auto_increment primary key,

  company_id bigint unsigned not null,

  full_name varchar(255) not null,
  date_of_birth date null,
  nationality varchar(64) null,
  country char(2) null,
  occupation varchar(128) null,

  appointed_on date null,
  correspondence_address text null,

  role varchar(128) not null,
  status enum ('active', 'inactive') null,

  created_at timestamp default current_timestamp,

  foreign key (company_id) references companies(id) on delete cascade
);


create table if not exists kyb_company_psc (
  id bigint unsigned auto_increment primary key,

  company_id bigint unsigned not null,

  full_name varchar(255) not null,
  date_of_birth date null,
  nationality varchar(64) null,
  country char(2) null,
  occupation varchar(128) null,

  appointed_on date null,
  correspondence_address text null,

  nature_of_control varchar(255) not null,   -- ownership, voting rights, etc
  status enum ('active', 'inactive') null,

  created_at timestamp default current_timestamp,

  foreign key (company_id) references companies(id) on delete cascade
);


create table if not exists kyb_sources (
  id bigint unsigned auto_increment primary key,

  company_id bigint unsigned not null,

  source_name varchar(255) not null,    -- Companies House, IRS, AFIP, etc
  source_url text not null,

  created_at timestamp default current_timestamp,

  foreign key (company_id) references companies(id) on delete cascade
);



create table if not exists wallets (
  id bigint unsigned auto_increment primary key,
  
  user_id bigint unsigned not null,

  encrypted_entropy blob not null,
  entropy_nonce binary(12) not null,
  
  encrypted_dek blob not null,
  dek_nonce binary(12) not null,
  
  kek_version smallint unsigned not null default 1,
  
  cipher_alg varchar(32) not null default 'AES-256-GCM',
  
  created_at timestamp default current_timestamp,
  updated_at timestamp default current_timestamp on update current_timestamp,
  
  foreign key (user_id) references users(id) on delete cascade
);

create table if not exists kyc (
  id bigint unsigned auto_increment primary key,

  user_id bigint unsigned not null,
  subject_type enum ('individual', 'company') not null default 'individual',
  risk_score int unsigned not null default 0,
  
  status enum ('not_started','pending', 'rejected', 'approved', 'resubmission_required') not null default 'not_started',
  type_kyc enum ('basic', 'identity', 'address', 'enhanced', 'corporate', 'ongoing') not null default 'basic',

  rejection_reason text null,
  
  provider varchar(64) not null default 'sandbox',
  provider_reference varchar(255) null,
  
  created_at timestamp default current_timestamp,
  updated_at timestamp default current_timestamp on update current_timestamp,
  
  unique key uk_kyc_user (user_id),
  foreign key (user_id) references users(id) on delete cascade
);


create table if not exists kyc_events (
  id bigint unsigned auto_increment primary key,

  kyc_id bigint unsigned not null,

  old_status enum ('not_started','pending','rejected','approved', 'resubmission_required') not null,

  new_status enum ('not_started','pending','rejected','approved', 'resubmission_required') not null,

  reason text null,

  provider varchar(64) not null,
  provider_reference varchar(255) null,

  created_at timestamp not null default current_timestamp,

  foreign key (kyc_id) references kyc(id) on delete cascade
);


create table risk_factors (
  id bigint unsigned auto_increment primary key,
  
  code varchar(64) unique not null,
  description varchar(255),
  weight int not null,
  active bool default true
);


create table kyc_risk_factors (
  id bigint unsigned auto_increment primary key,
  
  kyc_id bigint unsigned not null,
  
  risk_factor_id bigint unsigned not null,
  applied bool not null,
  created_at timestamp default current_timestamp,

  foreign key (kyc_id) references kyc(id) on delete cascade,
  foreign key (risk_factor_id) references risk_factors(id)
);

create table kyc_risk_events (
  id bigint unsigned auto_increment primary key,
  kyc_id bigint unsigned not null,

  old_score int,
  new_score int,

  reason text not null,
  created_at timestamp default current_timestamp,

  foreign key (kyc_id) references kyc(id) on delete cascade
);


create table if not exists kyc_documents (
  id bigint unsigned auto_increment primary key,

  kyc_id bigint unsigned not null,

  document_type enum ('passport', 'national_id', 'driver_license', 'proof_of_address', 'selfie', 'selfie_with_document', 'company_registration', 'shareholders_register', 'other') not null,

  file_name varchar(255) not null,
  file_mime varchar(64) not null,
  file_size bigint unsigned not null,

  storage_provider enum ('local', 's3', 'gcs', 'provider') not null default 'local',
  storage_path text not null,   -- path, S3 key, provider reference

  file_hash char(64) not null,  -- SHA-256 (integrity + legal)

  status enum ('uploaded', 'pending_review', 'approved', 'rejected') not null default 'uploaded',

  rejection_reason text null,

  uploaded_at timestamp default current_timestamp,
  reviewed_at timestamp null,

  foreign key (kyc_id) references kyc(id) on delete cascade
);
