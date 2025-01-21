-- Create tables
CREATE TABLE bank_status (
                             status_id bigint PRIMARY KEY,
                             status_name varchar(50)
);

CREATE TABLE banks (
                       bank_id bigint IDENTITY(1,1) PRIMARY KEY,
                       bank_name varchar(50),
                       bank_code varchar(50) UNIQUE,
                       contact_name varchar(50),
                       contact_mail varchar(50),
                       notification_mail varchar(50),
                       contact_phone varchar(15),
                       bank_ruc varchar(30),
                       status_id bigint,
                       FOREIGN KEY (status_id) REFERENCES bank_status(status_id)
);

CREATE TABLE accounts (
                          account_id bigint IDENTITY(1,1) PRIMARY KEY,
                          account_number varchar(100),
                          bank_id bigint,
                          FOREIGN KEY (bank_id) REFERENCES banks(bank_id)
);

CREATE TABLE commerce_status (
                                 commerce_status_id bigint PRIMARY KEY,
                                 status_name varchar(50)
);

CREATE TABLE alias_type (
                            alias_type_id bigint PRIMARY KEY,
                            description varchar(100)
);

CREATE TABLE commerces (
                           id_commerce bigint IDENTITY(1,1) PRIMARY KEY,
                           alias varchar(400),
                           alias_type_id bigint,
                           legal_business_name varchar(400),
                           account_id bigint,
                           ruc varchar(50),
                           commerce_status_id bigint,
                           FOREIGN KEY (alias_type_id) REFERENCES alias_type(alias_type_id),
                           FOREIGN KEY (account_id) REFERENCES accounts(account_id),
                           FOREIGN KEY (commerce_status_id) REFERENCES commerce_status(commerce_status_id)
);

-- Insert status data
INSERT INTO commerce_status (commerce_status_id, status_name) VALUES
                                                                  (1, 'ACTIVE'),
                                                                  (2, 'INACTIVE');

INSERT INTO alias_type (alias_type_id, description) VALUES
                                                        (1, 'PHONE'),
                                                        (2, 'COMMERCE');

INSERT INTO bank_status (status_id, status_name) VALUES
                                                     (1, 'ENABLED'),
                                                     (2, 'DISABLED');

-- Insert sample bank data
INSERT INTO banks (bank_name, bank_code, contact_name, contact_mail, notification_mail, contact_phone, bank_ruc, status_id) VALUES
                                                                                                                                ('Bank of America', 'BOA001', 'John Doe', 'john.doe@boa.com', 'notifications@boa.com', '1234567890', '12345678901', 1),
                                                                                                                                ('Chase Bank', 'CHASE001', 'Jane Smith', 'jane.smith@chase.com', 'notifications@chase.com', '9876543210', '98765432109', 1),
                                                                                                                                ('Wells Fargo', 'WF001', 'Bob Johnson', 'bob.j@wellsfargo.com', 'notices@wellsfargo.com', '5555555555', '55555555555', 1);

-- Insert sample account data with UUID v4
INSERT INTO accounts (account_number, bank_id) VALUES
                                                   ('123e4567-e89b-12d3-a456-426614174000', 1),
                                                   ('987fcdeb-51a2-42d3-a456-426614174001', 2),
                                                   ('550e8400-e29b-41d4-a716-446655440000', 3);

-- Update some fields
UPDATE dbo.banks SET bank_code='001' WHERE bank_id=1;
UPDATE dbo.banks SET bank_code='002' WHERE bank_id=2;
UPDATE dbo.banks SET bank_code='003' WHERE bank_id=3;
UPDATE dbo.banks SET status_id=2 WHERE bank_id=3;