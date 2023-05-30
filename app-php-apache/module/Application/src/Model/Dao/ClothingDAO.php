<?php

namespace Application\Model\Dao;

use Application\Model\Clothing;
use Laminas\Db\Adapter\AdapterInterface;
use Laminas\Db\ResultSet\ResultSet;
use Laminas\Db\TableGateway\TableGateway;
use Laminas\Db\TableGateway\TableGatewayInterface;

class ClothingDAO extends TableGateway {

    public function __construct(AdapterInterface $adapter) {
        
        $resultSetPrototype = new ResultSet();
        $resultSetPrototype->setArrayObjectPrototype(new Clothing());

        parent::__construct('clothing', $adapter, null, $resultSetPrototype);
    }

    public function getClothingById(int $id) {
        $sql = "SELECT * FROM clothing WHERE id = ?";
        $statement = $this->adapter->createStatement($sql);
        $result = $statement->execute([$id]);
        $row = $result->current();
        return $row;
    }
}