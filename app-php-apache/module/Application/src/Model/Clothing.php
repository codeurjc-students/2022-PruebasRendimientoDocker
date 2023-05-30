<?php

namespace Application\Model;

class Clothing extends \ArrayObject {
    
    public ?int $id;
    public ?string $file_name;
    public ?string $label;
    public ?string $size;
    public ?bool $kids;

    public function exchangeArray(object|array $data): array {
        $this->id = $data['id'] ?? null;
        $this->file_name = $data['file_name'] ?? null;
        $this->label = $data['label'] ?? null;
        $this->size = $data['size'] ?? null;
        $this->kids = $data['kids'] ?? null;

        return [
            'id' => $this->id,
            'file_name' => $this->file_name,
            'label' => $this->label,
            'size' => $this->size,
            'kids' => $this->kids,
        ];
    }
}