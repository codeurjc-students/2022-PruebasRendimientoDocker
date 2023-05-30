<?php

namespace Application\Controller\Factory;
use Application\Controller\IndexController;
use Application\Service\ClothingService;
use Laminas\ServiceManager\Factory\FactoryInterface;
use Psr\Container\ContainerInterface;

class IndexControllerFactory implements FactoryInterface {

    function __invoke(
        ContainerInterface $container,
        $requestedName,
        ?array $options = null
    ) {
        /** @var ClothingService $clothingService */
        $clothingService = $container->get(ClothingService::class);
        
        return new IndexController($clothingService);
    }
}