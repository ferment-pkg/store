type Barrell = {
    name: string;
    description: string;
    download: string;
    git: boolean;
    dependencies?: string[];
    home?: string;
    downloads:number;
};
type Settings={
    packageManagerPath: string;
    apiPath: string;
    installOptions: string[];
}