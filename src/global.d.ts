type Barrell = {
    name: string;
    description: string;
    download: string;
    git: boolean;
    dependencies?: string[];
    home?: string;
};
type Settings={
    packageManagerPath: string;
    apiPath: string;
    installOptions: string[];
}