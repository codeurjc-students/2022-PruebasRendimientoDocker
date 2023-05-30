<?php

namespace Application\Service;

use Application\Model\Dao\ClothingDAO;

class ClothingService
{
    private ClothingDAO $clothingDAO;

    public function __construct(ClothingDAO $clothingDAO) {
        $this->clothingDAO = $clothingDAO;
    }

    public function getClothingById(int $id) {
        return $this->clothingDAO->getClothingById($id);
    }
}

