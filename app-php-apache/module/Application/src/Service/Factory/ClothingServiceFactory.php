<?php

namespace Application\Service\Factory;

use Application\Model\Dao\ClothingDAO;
use Application\Service\ClothingService;
use Laminas\Db\Adapter\Adapter;
use Laminas\Db\Adapter\AdapterInterface;
use Laminas\ServiceManager\Factory\FactoryInterface;
use Psr\Container\ContainerInterface;

class ClothingServiceFactory implements FactoryInterface {

    function __invoke(
        ContainerInterface $container,
        $requestedName,
        ?array $options = null
    ) {
        /** @var AdapterInterface $dbAdapter */
        $dbAdapter = $container->get(AdapterInterface::class);
        
        $clothingDAO = new ClothingDAO($dbAdapter);
        return new ClothingService($clothingDAO);
    }
}