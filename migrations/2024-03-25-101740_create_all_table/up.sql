-- admin
/*
警告: 表名可能非法 - admin
警告: 字段名可能非法 - password
*/
create table  admin
(
       id                VARCHAR(36) not null comment 'ID',
       role_id           VARCHAR(36) comment '对应角色',
       company_id        VARCHAR(36) comment '所属公司',
       username          VARCHAR(20) comment '账号',
       password          VARCHAR(100) not null comment '密码',
       phone             VARCHAR(20) not null comment '手机号',
       avatar            VARCHAR(4000) comment '头像',
       nickname          VARCHAR(20) comment '用户名'
) comment '管理员';
alter  table admin
       add constraint PK_admin_id primary key (id);
create unique index IDU_admin_username on admin(username);

-- role
/*
警告: 表名可能非法 - role
*/
create table  role
(
       id                VARCHAR(36) not null comment '角色ID',
       rolename          VARCHAR(20) not null comment '角色名',
       description       VARCHAR(100) comment '角色说明',
       router            VARCHAR(4000) comment '路由',
       admin_value       INT not null,
       operator_value    INT not null,
       role_value        INT not null,
       company_value     INT not null,
       salesman_value    INT not null,
       sales_records_value INT not null,
       product_value     INT not null,
       custom_value      INT not null
) comment '角色';
alter  table role
       add constraint PK_role_id primary key (id);

-- company
create table  company
(
       id                VARCHAR(36) not null comment '编号',
       name              VARCHAR(20) comment '公司名'
) comment '公司';
alter  table company
       add constraint PK_company_id primary key (id);
create unique index IDU_company_name on company(name);

-- salesman
create table  salesman
(
       id                INT primary key auto_increment not null comment '编号',
       company_id        VARCHAR(36) comment '所属公司',
       username          VARCHAR(20) not null comment '销售员姓名',
       phone             VARCHAR(20) not null comment '销售员电话'
) comment '销售员';
create unique index IDU_salesman_phone on salesman(phone);

-- custom
/*
警告: 字段名可能非法 - password
警告: 字段名可能非法 - level
*/
create table  custom
(
       id                INT primary key auto_increment not null comment '编号',
       name              VARCHAR(10) not null comment '姓名',
       phone             VARCHAR(20) comment '手机号',
       password          VARCHAR(100) not null comment '密码',
       id_type           VARCHAR(20) comment '证件类型',
       id_number         VARCHAR(50) comment '证件号',
       level             INT default 1 comment '用户级别'
) comment '客户';
create unique index IDU_custom_phone on custom(phone);

-- custom_salesman
create table  custom_salesman
(
       id                INT primary key auto_increment not null comment '系统内部id',
       custom_id         INT comment '客户_编号',
       salesman_id       INT comment '销售员_编号',
       product_id        VARCHAR(36) comment '产品_编号',
       create_at         TIMESTAMP not null comment '创建时间',
       company           VARCHAR(20) not null comment '所属公司 销售员调到其他公司后，该记录应该属于是公司的',
       order_id          VARCHAR(30) not null comment '订单号 公司内部负责',
       pay_method        VARCHAR(30) not null comment '支付方式',
       money             decimal not null comment '支付金额 decimal',
       people_number     INT not null comment '人数',
       rebate            VARCHAR(50) comment '返点 提成'
) comment '销售记录';

-- operator
/*
警告: 表名可能非法 - operator
*/
create table  operator
(
       id                INT primary key auto_increment not null comment '编号',
       admin_id          VARCHAR(36) comment '操作者',
       teablename        VARCHAR(30) not null comment '表名',
       source_id         VARCHAR(32) not null comment '操作的对象ID',
       created_at        TIMESTAMP not null comment '操作时间',
       operator_type     VARCHAR(20) not null comment '操作类型',
       origin_object     VARCHAR(4000) comment '操作前数据',
       now_object        VARCHAR(4000) comment '操作后数据',
       notes             VARCHAR(300) comment '备注'
) comment '操作记录';

-- 分组21

-- 分组22

-- product
create table  product
(
       id                VARCHAR(36) not null comment '本身id',
       base_product_id   VARCHAR(36) comment '对应基础产品id',
       create_at         TIMESTAMP not null comment '创建时间',
       price             decimal default 0 comment '价格 decimal',
       start_time        TIMESTAMP not null comment '开始时间',
       end_time          TIMESTAMP not null comment '结束时间',
       people_number     INT not null comment '人数 一次最多几个人',
       duration          INT not null comment '团期',
       product_type      VARCHAR(20) not null comment '销售类型 研学游、避暑游、直通车、主题游、夏令营、冬令营、代理服务、其他',
       notes             VARCHAR(500) comment '备注'
) comment '实际产品';
alter  table product
       add constraint PK_product_id primary key (id);

-- base_product
create table  base_product
(
       id                VARCHAR(36) not null,
       create_at         TIMESTAMP not null comment '创建时间',
       name              VARCHAR(50) comment '产品名',
       file_list         VARCHAR(500) comment '产品海报',
       notes             VARCHAR(500) comment '备注'
) comment '基础产品';
alter  table base_product
       add constraint PK_base_product_id primary key (id);
create unique index IDU_base_product_name on base_product(name);

-- ledger
/*
警告: 字段名可能非法 - cost
*/
create table  ledger
(
       id                VARCHAR(36) not null,
       product_name      VARCHAR(50) not null comment '产品名称',
       start_time        TIMESTAMP not null comment '产品开始时间',
       end_time          TIMESTAMP not null comment '产品结束时间',
       people_number     INT not null comment '产品总计人数',
       product_type      VARCHAR(20) not null comment '产品类型',
       duration          INT not null comment '产品团期',
       revenue           decimal not null comment '收入',
       cost              decimal not null comment '成本',
       pay_status        VARCHAR(20) not null comment '结账情况 可能会存在未结账情况',
       executor          VARCHAR(20) not null comment '执行人',
       notes             VARCHAR(500) comment '备注'
) comment '台账记录 由产品销售情况结算生成而来';
alter  table ledger
       add constraint PK_ledger_id primary key (id);

-- 分组26

-- 客户

alter  table admin
       add constraint FK_admin_role_id foreign key (role_id)
       references role(id);
alter  table admin
       add constraint FK_admin_company_id foreign key (company_id)
       references company(id);

alter  table salesman
       add constraint FK_salesman_company_id foreign key (company_id)
       references company(id);

alter  table custom_salesman
       add constraint FK_custom_man_custom_id179D foreign key (custom_id)
       references custom(id);
alter  table custom_salesman
       add constraint FK_custom_man_salesma_id3D2F foreign key (salesman_id)
       references salesman(id);
alter  table custom_salesman
       add constraint FK_custom_man_product_id9D56 foreign key (product_id)
       references product(id);

alter  table operator
       add constraint FK_operator_admin_id foreign key (admin_id)
       references admin(id);

alter  table ledger
       add constraint FK_product_id foreign key (id)
       references product(id);
alter  table product
       add constraint FK_base_product_id foreign key (base_product_id)
       references base_product(id);

